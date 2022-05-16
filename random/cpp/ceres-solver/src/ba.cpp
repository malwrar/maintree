#include <math.h>
#include <iostream>

#include <ceres/ceres.h>
#include <ceres/rotation.h>
#include <matplot/matplot.h>

#include "bal.h"

// Templated pinhole camera model for used with Ceres.  The camera is
// parameterized using 9 parameters: 3 for rotation, 3 for translation, 1 for
// focal length and 2 for radial distortion. The principal point is not modeled
// (i.e. it is assumed be located at the image center).
struct SnavelyReprojectionError {
    SnavelyReprojectionError(double observed_x, double observed_y)
            : observed_x(observed_x), observed_y(observed_y) {}

    template <typename T>
    bool operator()(
        const T* const camera,
        const T* const point,
        T* residuals
    ) const {
        // camera[0,1,2] are the angle-axis rotation.
        T p[3];
        ceres::AngleAxisRotatePoint(camera, point, p);

        // camera[3,4,5] are the translation.
        p[0] += camera[3];
        p[1] += camera[4];
        p[2] += camera[5];

        // Compute the center of distortion. The sign change comes from
        // the camera model that Noah Snavely's Bundler assumes, whereby
        // the camera coordinate system has a negative z axis.
        T xp = -p[0] / p[2];
        T yp = -p[1] / p[2];

        // Apply second and fourth order radial distortion.
        const T& l1 = camera[7];
        const T& l2 = camera[8];
        T r2 = xp * xp + yp * yp;
        T distortion = 1.0 + r2 * (l1 + l2 * r2);

        // Compute final projected point position.
        const T& focal = camera[6];
        T predicted_x = focal * distortion * xp;
        T predicted_y = focal * distortion * yp;

        // The error is the difference between the predicted and observed position.
        residuals[0] = predicted_x - observed_x;
        residuals[1] = predicted_y - observed_y;

        return true;
    }

    // Factory to hide the construction of the CostFunction object from
    // the client code.
    static ceres::CostFunction* Create(
        const double observed_x,
        const double observed_y
    ) {
        return (
            new ceres::AutoDiffCostFunction<SnavelyReprojectionError, 2, 9, 3>(
                    new SnavelyReprojectionError(observed_x, observed_y)));
    }

    double observed_x;
    double observed_y;
};

int main(int argc, char** argv) {
    google::InitGoogleLogging(argv[0]);
    if (argc != 2) {
        std::cerr << "usage: " << argv[0] << " <bal_problem.txt>\n";
        return 1;
    }

    BALProblem bal_problem;
    if (!bal_problem.LoadFile(argv[1])) {
        std::cerr << "ERROR: unable to open file " << argv[1] << "\n";
        return 1;
    }
    const double* observations = bal_problem.observations();

    // Create residuals for each observation in the bundle adjustment problem. The
    // parameters for cameras and points are added automatically.
    ceres::Problem problem;
    for (int i = 0; i < bal_problem.num_observations(); ++i) {
        // Each Residual block takes a point and a camera as input and outputs a 2
        // dimensional residual. Internally, the cost function stores the observed
        // image location and compares the reprojection against the observation.
        ceres::CostFunction* cost_function = SnavelyReprojectionError::Create(
            observations[2 * i + 0], observations[2 * i + 1]);

        problem.AddResidualBlock(cost_function, nullptr /* squared loss */,
                bal_problem.mutable_camera_for_observation(i),
                bal_problem.mutable_point_for_observation(i));
    }

    // Make Ceres automatically detect the bundle structure. Note that the
    // standard solver, SPARSE_NORMAL_CHOLESKY, also works fine but it is slower
    // for standard bundle adjustment problems.
    ceres::Solver::Options options;
    options.linear_solver_type = ceres::DENSE_SCHUR;
    options.minimizer_progress_to_stdout = true;

    ceres::Solver::Summary summary;

    ceres::Solve(options, &problem, &summary);


    // Output 3d positions of points.
    std::vector<double> x, y, z;
    std::vector<double> sizes;
    for (int i = 0; i < bal_problem.num_observations(); ++i) {
        //double* pose = bal_problem.mutable_camera_for_observation(i);
        double* point = bal_problem.mutable_point_for_observation(i);
	static const double MAX_DIST = 10000;
	if (fabs(point[0]) < MAX_DIST
                && fabs(point[1]) < MAX_DIST
                && fabs(point[2]) < MAX_DIST) {
            x.push_back(point[0]);
            y.push_back(point[1]);
            z.push_back(point[2]);
            sizes.push_back(1.0);
            
            //printf("%.2f %.2f %.2f\n", point[0], point[1], point[2]);
	}
    }

    printf("total poses: %lu\n", x.size());

    matplot::scatter3(x, y, z, sizes);
    matplot::show();

    std::cout << summary.FullReport() << "\n";

    return 0;
}
