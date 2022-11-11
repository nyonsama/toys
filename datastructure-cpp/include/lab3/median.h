#ifndef __MEDIAN_H__
#define __MEDIAN_H__

#include<vector>
#include<fmt/core.h>
#include<cstdint>

namespace lab3 {
	namespace median {
		// x and y are in ascending order
		double get_median(double* x,  double* y, size_t n);

		// x and y are in ascending order
		double get_median(std::vector<double>& x, std::vector<double>& y);
	}
}

#endif // !__MEDIAN_H__
