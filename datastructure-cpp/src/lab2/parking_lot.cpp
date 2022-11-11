#include"lab2/parking_lot.h"
#include"fmt/core.h"

using linked_list::LinkedList;
using std::pair;
using std::make_pair;
using std::string;
using std::vector;
using std::sscanf;
using fmt::print;

namespace lab2 {
	namespace parking_lot {
		ParkingLot::ParkingLot(size_t cap) :cap_(cap), space_(LinkedList<pair<size_t, size_t>>()), queue_(LinkedList<size_t>()) {}
		ParkingLot::~ParkingLot() {}

		pair<Status, size_t> ParkingLot::arrive(size_t car_id, size_t cur_time) {
			size_t len = this->space_.len();
			if (len > this->cap_) {
				print("parkinglot stack overflow!\n");
				exit(1);
			}
			else if (len == this->cap_) {
				this->queue_.push_back(car_id);
				return make_pair(Status::Waiting, car_id);
			}
			else {
				this->space_.push_back(make_pair(car_id, cur_time));
				return make_pair(Status::Parking, car_id);
			}
		}

		pair<size_t, size_t> ParkingLot::leave(size_t car_id, size_t cur_time) {
			auto iter = this->space_.iter();
			size_t duration = cur_time;
			for (size_t i = 0; i < this->space_.len(); i++) {
				auto p = iter->next();
				if (p.first == car_id) {
					duration -= this->space_.remove(i).value().second;
					// auto id = this->queue_.pop_front();
					if (auto id = this->queue_.pop_front(); id.has_value())
						this->space_.push_back(make_pair(id.value(), cur_time));
					break;
				}
			}
			return make_pair(duration, duration * 2);
		}

		void ParkingLot::run(size_t cap, string input) {
			ParkingLot p(cap);
			input.pop_back();
			input.erase(0, 1);
			vector<string> subs;

			// split
			size_t start = 0;
			size_t end = 0;
			while ((end = input.find("),(", start)) != string::npos) {
				subs.push_back(input.substr(start, end - start));
				start = end + 3;
			}
			subs.push_back(input.substr(start));

			for (string s : subs) {
				char action;
				size_t car_id;
				size_t cur_time;
				sscanf(s.c_str(), "'%c',%zd,%zd", &action, &car_id, &cur_time);
				switch (action) {
				case 'A': {
					auto status_carid = p.arrive(car_id, cur_time);
					switch (status_carid.first) {
					case Status::Parking:
						print("time:{:<2} car {} arrived, status:Parking\n", cur_time, car_id);
						break;
					case Status::Waiting:
						print("time:{:<2} car {} arrived, status:Waiting\n", cur_time, car_id);
						break;
					default:
						print("undefinded status!\n");
						exit(1);
					}
					break;
				}
				case 'D': {
					auto duration_fee = p.leave(car_id, cur_time);
					print("time:{:<2} car {} leaved,  duration:{:>2}, fee:{:<2}\n", cur_time, car_id, duration_fee.first, duration_fee.second);
					break;
				}
				case 'E':
					print("exiting\n");
					break;
				default:
					print("undefinded operation!\n");
					exit(1);
				}
			}
		}
	}
}
