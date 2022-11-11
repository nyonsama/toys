#ifndef __MAZE_H__
#define __MAZE_H__

#include<cstdint>
#include<vector>
#include<tuple>
#include<utility>
#include<fmt/core.h>
#include<fmt/format.h>
#include"linked_list.h"

namespace lab2 {

	namespace maze {
		enum class Direction {
			Unknown, // 还没走
			Right,
			Down,
			Left,
			Up,
			Impasse  // 死路
		};

		//				     row, column
		typedef std::pair<size_t, size_t> Position;
		typedef struct PathPoint {
			size_t row;
			size_t column;
			Direction to;
			//Direction last_direction;
			operator std::string()const {
				char direction;
				switch (this->to) {
				case Direction::Unknown:
					direction = '_'; break;
				case Direction::Right:
					direction = '1'; break;
				case Direction::Down:
					direction = '2'; break;
				case Direction::Left:
					direction = '3'; break;
				case Direction::Up:
					direction = '4'; break;
				default:
					fmt::print("invalid direction dectected!");
					exit(1);
				}
				return fmt::format("({}, {}, {})", this->row + 1, this->column + 1, direction);
			}
		} PathPoint;


		class Maze {
		public:
			Maze(size_t width, size_t height, std::vector<std::vector<uint8_t>> map, Position entrance, Position exit);
			~Maze();
			linked_list::LinkedList<PathPoint> run();

		private:
			std::vector<std::vector<uint8_t>> map_;
			size_t width_;
			size_t height_;
			Position entrance_;
			Position exit_;
			std::optional<PathPoint> step(PathPoint& cur);
		};

		/*
		Maze::Maze() {
			this->maze_ = vector<uint8_t>{
				0, 0, 1, 0, 0, 0, 1, 0,
				0, 0, 1, 0, 0, 0, 1, 0,
				0, 0, 0, 0, 1, 1, 0, 1,
				0, 1, 1, 1, 0, 0, 1, 0,
				0, 0, 0, 1, 0, 0, 0, 0,
				0, 1, 0, 0, 0, 1, 0, 1,
				0, 1, 1, 1, 1, 0, 0, 1,
				1, 1, 0, 0, 0, 1, 0, 1,
				1, 1, 1, 0, 0, 0, 0, 0
			};
		}*/

	}
}

template <> struct fmt::formatter<lab2::maze::PathPoint> : formatter<string_view> {
	// parse is inherited from formatter<string_view>.
	template <typename FormatContext>
	auto format(lab2::maze::PathPoint c, FormatContext& ctx) {
		return formatter<string_view>::format(string_view(std::string(c)), ctx);
	}
};

#endif // !__MAZE_H__
