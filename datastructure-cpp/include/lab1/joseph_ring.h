#ifndef __JOSEPH_RING_H__
#define __JOSEPH_RING_H__

#include"linked_list.h"
#include<utility>
#include<vector>
#include<memory>

namespace lab1 {
	namespace joseph_ring {
		typedef std::pair<size_t, size_t> IdPass;
		class JosephRing
		{
		public:
			JosephRing(size_t size, size_t limit, std::vector<size_t> password);
			~JosephRing();
			linked_list::LinkedList<size_t> run();

		private:
			size_t limit;
			std::shared_ptr<linked_list::LinkedList<IdPass>> ring;

		};
	}
}
#endif // !__JOSEPH_RING_H__

