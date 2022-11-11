#include "lab1/joseph_ring.h"

using linked_list::LinkedList;
using std::make_shared;
using std::vector;

namespace lab1 {
	namespace joseph_ring {
		JosephRing::JosephRing(size_t size, size_t limit, vector<size_t> password)
		{
			this->limit = limit;
			this->ring = make_shared<LinkedList<IdPass>>();
			for (size_t i = 0; i < size; i++)
			{
				this->ring->push_back(IdPass(i + 1, password.at(i)));
			}
		}

		JosephRing::~JosephRing()
		{
		}

		LinkedList<size_t> JosephRing::run() {
			LinkedList<size_t> result;
			auto m = this->limit;
			while (this->ring->len() != 0)
			{
				this->ring->rotate_left(m % this->ring->len());
				IdPass p = *this->ring->pop_back();
				m = p.second;
				result.push_back(p.first);
			}
			return result;
		}
	}
}