#pragma once

#include <cstdio>
#include <iostream>

#include <glog/logging.h>

// Read a Bundle Adjustment in the Large dataset.
class BALProblem {
public:
    ~BALProblem() {
        delete[] point_index_;
        delete[] camera_index_;
        delete[] observations_;
        delete[] parameters_;
    }

    int num_observations() const {
        return num_observations_;
    }

    const double* observations() const {
        return observations_;
    }

    double* mutable_cameras() {
        return parameters_;
    }

    double* mutable_points() {
        return parameters_ + 9 * num_cameras_;
    }

    double* mutable_camera_for_observation(int i) {
        return mutable_cameras() + camera_index_[i] * 9;
    }

    double* mutable_point_for_observation(int i) {
        return mutable_points() + point_index_[i] * 3;
    }

    bool LoadFile(const char* filename);

private:
    template <typename T>
    void FscanfOrDie(FILE* fptr, const char* format, T* value) {
        int num_scanned = fscanf(fptr, format, value);
        if (num_scanned != 1) {
            LOG(FATAL) << "Invalid UW data file.";
        }
    }

    int num_cameras_;
    int num_points_;
    int num_observations_;
    int num_parameters_;
    int* point_index_;
    int* camera_index_;
    double* observations_;
    double* parameters_;
};
