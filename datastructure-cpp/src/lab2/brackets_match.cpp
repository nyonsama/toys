#include"lab2/brackets_match.h"

using std::string;
using linked_list::LinkedList;
namespace lab2 {
	namespace brackets_match {
		bool brackets_match(string expr) {
			LinkedList<char> stack;
			for (char c : expr) {
				switch (c) {
				case '(':
				case '[':
				case '{':
					stack.push_back(c);
					break;
				case ')':
				case ']':
				case '}':
					if (auto l = stack.pop_back(); l.has_value()) {
						if (char d = c - *l; d != 1 && d != 2) {
							return false;
						}
					}
					else
						return false;
					break;
				default:
					break;
				}
			}
			if (stack.len() != 0)
				return false;
			else
				return true;
		}
	}
}