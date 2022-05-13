#include <math.h>

#include <set>
#include <vector>

#include <ceres/ceres.h>
#include <glog/logging.h>
#include <matplot/matplot.h>

using ceres::NumericDiffCostFunction;
using ceres::CostFunction;
using ceres::Problem;
using ceres::Solve;
using ceres::Solver;

struct Fn {
    bool operator()(const double* const x, double* residual) const {
        residual[0] = 0.5 * pow(10.0 - x[0], 2);
        return true;
    }
};

int main(int argc, char** argv) {
    google::InitGoogleLogging(argv[0]);

    // The variable to solve for with its initial value. It will be
    // mutated in place by the solver.
    double x = 1.0;
    const double initial_x = x;

    // Build the problem.
    Problem problem;

    // Set up the only cost function (also known as residual). This uses
    // numeric differentiation to obtain the derivative (jacobian).
    CostFunction* cost_function =
            new NumericDiffCostFunction<Fn, ceres::CENTRAL, 1, 1>(new Fn);
    problem.AddResidualBlock(cost_function, nullptr, &x);

    // Run the solver!
    Solver::Options options;
    options.minimizer_progress_to_stdout = true;
    Solver::Summary summary;
    Solve(options, &problem, &summary);

    std::cout << summary.BriefReport() << "\n";
    std::cout << "x : " << initial_x << " -> " << x << "\n";

    std::set<std::vector<double>> y = {
        {16, 5, 9, 4}, {2, 11, 7, 14}, {3, 10, 6, 15}, {13, 8, 12, 1}};
    matplot::plot(y);

    matplot::show();
    return 0;
}
