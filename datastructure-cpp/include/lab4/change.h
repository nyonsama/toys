#ifndef __CHANGE_H__
#define __CHANGE_H__

#include<vector>
namespace lab4 {
	namespace change {
		size_t change(double total, std::vector<double> cashes);
		size_t change(double total, double* cashes, size_t cashes_len);
	}
}

#endif // !__CHANGE_H__
