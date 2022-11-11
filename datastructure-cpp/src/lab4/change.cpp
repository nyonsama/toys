#include"lab4/change.h"
#include<algorithm>

using std::min;
namespace lab4 {
	namespace change {
		size_t change(double total, std::vector<double> cashes) {
			return change(total, cashes.data(), cashes.size());
		}
		size_t change(double total, double* cashes, size_t cashes_len) {
			size_t min_cost = SIZE_MAX;
			for (size_t i = 0; i < cashes_len; i++) {
				if (total >= cashes[i]) {
					min_cost = min(min_cost, change(total - cashes[i], cashes, cashes_len));
				}
			}
			if (min_cost == SIZE_MAX) { // 没有合适的纸币了
				if (total == 0) 
					return 0; // 找完了
				else 
					return 1; // 找不出来
			}
			else {
				return min_cost + 1;
			}
		}
	}
}
