#ifndef __PARKING_LOT_H__
#define __PARKING_LOT_H__

#include"linked_list.h"
#include<cstdio>
#include<string>
#include<vector>
#include<utility>

namespace lab2 {

	namespace parking_lot {
		enum class Status {
			Parking,
			Waiting
		};

		class ParkingLot {
		public:
			ParkingLot(size_t cap);
			~ParkingLot();

			//		  status, car_id
			std::pair<Status, size_t> arrive(size_t car_id, size_t cur_time);
			//          time, fee
			std::pair<size_t, size_t> leave(size_t car_id, size_t cur_time);

			static void run(size_t cap, std::string input);

		private:
			size_t cap_;
			//								  car_id, cur_time
			linked_list::LinkedList<std::pair<size_t, size_t>> space_;
			linked_list::LinkedList<size_t> queue_;
		};
	}

}
#endif // !__PARKING_LOT_H__
