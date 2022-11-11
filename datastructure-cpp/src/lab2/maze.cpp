#include"lab2/maze.h"
#include<fmt/core.h>

using std::vector;
using std::make_tuple;
using std::make_pair;
using std::make_optional;
using std::optional;
using std::nullopt;
using std::get;
using linked_list::LinkedList;

namespace lab2 {
	namespace maze {
		// 将以(1,1)为原点的坐标转换为以(0,0)为原点的坐标
		Maze::Maze(size_t width, size_t height, vector<vector<uint8_t>> map, Position entrance, Position exit) :
			width_(width), height_(height), map_(map),
			entrance_(Position(entrance.first - 1, entrance.second - 1)),
			exit_(Position(exit.first - 1, exit.second - 1)) {
			/*
			this->map_ = vector<vector<uint8_t>>();
			for (size_t i = 0; i < height; i++) {
				this->map_.push_back(vector<uint8_t>());
				for (size_t j = 0; j < width; j++) {
					this->map_[i].push_back(map[i * width + j]);
				}
			}
			*/
		}

		Maze::~Maze() {}

		LinkedList<PathPoint> Maze::run() {
			LinkedList<PathPoint> path;
			PathPoint cur = {
				this->entrance_.first,
				this->entrance_.second,
				Direction::Unknown
			};
			for (size_t i = 0; i < this->height_ * this->width_; i++) {
				/*
				获得下一步，压栈，判断终点，获得下一步，压栈，判断终点...
				没有下一步就弹栈，获得上一步的下一个方向
				*/
				auto next = this->step(cur);
				if (next.has_value()) {
					// 走了一步
					path.push_back(cur);
					cur = *next;
					if (cur.row == this->exit_.first && cur.column == this->exit_.second) {
						// 走到了终点
						path.push_back(cur);
						break;
					}
				}
				else if (auto last = path.pop_back(); last.has_value()) {
					// 走进了死胡同
					cur = *last;
				}
				else {
					// 这个迷宫走不通
				}
			}
			return path;
		}


		// 站在cur的坐标，寻找下一个能走的方向，将cur的方向改为下一个能走的方向或Impasse(死路)
		// return向这个方向走一步之后的PathPoint(方向为None)
		optional<PathPoint> Maze::step(PathPoint& cur) {
			PathPoint next;
			size_t r = cur.row; // row
			size_t c = cur.column; // column
			this->map_[r][c] = 1; //已经走过的地方不能再走
			switch (cur.to) {
			case Direction::Unknown:
				// 在if后面加判断是不是走过来的方向
				if (size_t _c = c + 1; _c < this->width_ && this->map_[r][_c] == 0) { // right
					cur.to = Direction::Right;
					next = { r, _c, Direction::Unknown };
					break;
				}
			case Direction::Right:
				if (size_t _r = r + 1; _r < this->height_ && this->map_[_r][c] == 0) { // down
					cur.to = Direction::Down;
					next = { _r, c, Direction::Unknown };
					break;
				}
			case Direction::Down:
				if (size_t _c = c - 1; c > 0 && this->map_[r][_c] == 0) { // left
					cur.to = Direction::Left;
					next = { r, _c, Direction::Unknown };
					break;
				}
			case Direction::Left:
				if (size_t _r = r - 1; r > 0 && this->map_[_r][c] == 0) { // up
					cur.to = Direction::Up;
					next = { _r, c, Direction::Unknown };
					break;
				}
			case Direction::Up:
				cur.to = Direction::Impasse;
				return nullopt;
			default:
				fmt::print("incorrect direction!\n");
				exit(1);
			}
			return make_optional<PathPoint>(next);
		}
	}
}