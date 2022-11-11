#include"lab3/median.h"

namespace lab3 {
	namespace median {
		double get_median(double* x, double* y, size_t n) {
			switch (n) {
			case 0: {
				fmt::print("invalid length(0)!\n");
				exit(1);
			}
			case 1: {
				if (*x == *y)
					return *x;
				else
					return (*x + *y) / 2;
			}
			case 2: { //直接算出中位数
				double l, r;
				if (x[0] < y[0])
					l = y[0];
				else
					l = x[0];
				if (x[1] < y[1])
					r = x[1];
				else
					r = y[1];
				return (l + r) / 2;

			}
			default: {
				size_t i = n / 2;
				double xmid, ymid;
				if ((n & 1) == 1) {	//判断奇偶
					xmid = x[i];
					ymid = y[i];
				}
				else {
					xmid = (x[i - 1] + x[i]) / 2;
					ymid = (y[i - 1] + y[i]) / 2;
				}

				if (xmid > ymid)
					return get_median(x, &y[i], i + 1);
				else if (xmid < ymid)
					return get_median(&x[i], y, i + 1);
				else
					return xmid;
			}
			}
		}

		// x and y are in ascending order
		double get_median(std::vector<double>& x, std::vector<double>& y) {
			return get_median(x.data(), y.data(), y.size());
		}

	}
}

