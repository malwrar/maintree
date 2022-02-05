use bevy::{
    core_pipeline::Opaque3d,
    pbr::{DrawMesh, SetMeshBindGroup, SetMeshViewBindGroup, MeshPipeline, MeshPipelineKey, MeshUniform},
    reflect::TypeUuid,
    render::{
        RenderApp, RenderStage,
        mesh::Mesh,
        render_asset::RenderAssets,
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{PolygonMode, RenderPipelineDescriptor, RenderPipelineCache, Shader, SpecializedPipeline, SpecializedPipelines},
        view::{ExtractedView, Msaa},
    },
    prelude::*,
};
   
pub const POINTCLOUD_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2966306455104594990);

#[derive(Debug, Default)]
pub struct PointCloudPlugin;

impl Plugin for PointCloudPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            POINTCLOUD_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("pointcloud.wgsl")),
        );

        app.init_resource::<PointCloudConfig>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_command::<Opaque3d, DrawPointClouds>()
                .init_resource::<PointCloudPipeline>()
                .init_resource::<SpecializedPipelines<PointCloudPipeline>>()
                .add_system_to_stage(RenderStage::Extract, extract_pointclouds)
                .add_system_to_stage(RenderStage::Extract, extract_pointcloud_config)
                .add_system_to_stage(RenderStage::Queue, queue_pointclouds);
        }
    }
}

fn extract_pointcloud_config(mut commands: Commands, pointcloud_config: Res<PointCloudConfig>) {
    if pointcloud_config.is_added() || pointcloud_config.is_changed() {
        commands.insert_resource(pointcloud_config.into_inner().clone());
    }
}

fn extract_pointclouds(mut commands: Commands, query: Query<Entity, With<PointCloud>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(PointCloud);
    }
}

/// Controls whether an entity should rendered in pointcloud-mode if the [`PointCloudPlugin`] is enabled
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct PointCloud;

#[derive(Debug, Clone, Default)]
pub struct PointCloudConfig {
    /// Whether to show pointclouds for all meshes. If `false`, only meshes with a [PointCloud] component will be rendered.
    pub global: bool,
}

pub struct PointCloudPipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}
impl FromWorld for PointCloudPipeline {
    fn from_world(render_world: &mut World) -> Self {
        PointCloudPipeline {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: POINTCLOUD_SHADER_HANDLE.typed(),
        }
    }
}

impl SpecializedPipeline for PointCloudPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.polygon_mode = PolygonMode::Point;
        descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;
        descriptor
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_pointclouds(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    pointcloud_config: Res<PointCloudConfig>,
    pointcloud_pipeline: Res<PointCloudPipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<PointCloudPipeline>>,
    msaa: Res<Msaa>,
    mut material_meshes: QuerySet<(
        QueryState<(Entity, &Handle<Mesh>, &MeshUniform)>,
        QueryState<(Entity, &Handle<Mesh>, &MeshUniform), With<PointCloud>>,
    )>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<DrawPointClouds>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);

        let add_render_phase =
            |(entity, mesh_handle, mesh_uniform): (Entity, &Handle<Mesh>, &MeshUniform)| {
                if let Some(mesh) = render_meshes.get(mesh_handle) {
                    let key =
                        key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                    transparent_phase.add(Opaque3d {
                        entity,
                        pipeline: specialized_pipelines.specialize(
                            &mut pipeline_cache,
                            &pointcloud_pipeline,
                            key,
                        ),
                        draw_function: draw_custom,
                        distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                    });
                }
            };

        if pointcloud_config.global {
            material_meshes.q0().iter().for_each(add_render_phase);
        } else {
            material_meshes.q1().iter().for_each(add_render_phase);
        }
    }
}

type DrawPointClouds = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);