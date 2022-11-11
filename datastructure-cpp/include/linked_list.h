#ifndef __LINKED_LIST_H__
#define __LINKED_LIST_H__

#include<string>
#include<optional>
#include<memory>
#include<iostream>
#include<fmt/core.h>
#include<fmt/format.h>

namespace linked_list {
	template<typename T>
	class ListNode {
	public:
		ListNode(T content);
		ListNode(T content, ListNode<T>* prev, ListNode<T>* next);
		~ListNode();
		ListNode<T>* next_;
		ListNode<T>* prev_;
		T content;
	};

	template <typename T, typename U>
	class Iterator {
	public:
		Iterator(U* container) {
			this->container_ = container;
			this->cur_ = nullptr;
		}

		T& first() {
			return this->container_->head_->content;
		}

		T& next() {
			if (this->cur_ == nullptr)
				this->cur_ = this->container_->head_;
			else
				this->cur_ = this->cur_->next_;
			return this->cur_->content;
		}

		T& last() {
			if (this->cur_ == nullptr)
				this->cur_ = this->container_->tail_;
			else
				this->cur_ = this->cur_->prev_;
			return this->cur_->content;
		}

		bool is_done() {
			return this->cur_ == this->container_->tail_;
		}

		void reset() {
			this->cur_ = this->container_->head_;
		}

		T& current() {
			return this->cur_->content;
		}

	private:
		U* container_;
		ListNode<T>* cur_;
	};

	template <typename T>
	class LinkedList {
	friend class Iterator<T, LinkedList>;
	public:
		LinkedList();
		LinkedList(LinkedList<T>&& obj) noexcept;
		~LinkedList();
		operator std::string()const;

		void push_back(T item) {
			this->insert(this->len_, item);
		}

		void push_front(T item) {
			this->insert(0, item);
		}

		std::optional<T> pop_back() {
			return this->remove(this->len_ - 1);
		}

		std::optional<T> pop_front() {
			return this->remove(0);
		}

		void insert(size_t index, T item);
		std::optional<T> remove(size_t index);

		T& at(size_t index) const {
			ListNode<T>* node = this->head_;
			for (size_t i = 0; i < index; i++) {
				node = node->next_;
			}
			return node->content;
		}

		void rotate_left(size_t step) {
			for (size_t i = 0; i < step; i++) {
				this->head_ = this->head_->next_;
				this->tail_ = this->tail_->next_;
			}
		}

		size_t len()const {
			return this->len_;
		}

		std::shared_ptr<Iterator<T,LinkedList>> iter() {
			return std::make_shared<Iterator<T,LinkedList>>(this);
		}

		friend std::ostream& operator<<(std::ostream& output, const LinkedList<T>& L) {
			output << std::string(L);
			return output;
		}

	private:
		ListNode<T>* head_;
		ListNode<T>* tail_;
		size_t len_;
	};

	template<typename T>
	LinkedList<T>::LinkedList() :len_(0) {
		this->head_ = nullptr;
		this->tail_ = nullptr;
	}

	template<typename T>
	LinkedList<T>::~LinkedList() {
		if (this->head_ != nullptr) {
			size_t len = this->len_;
			ListNode<T>* cur = this->head_;
			for (size_t i = 0; i < len; i++) {
				auto last = cur;
				cur = cur->next_;
				delete last;
			}
		}
	}

	template<typename T>
	LinkedList<T>::LinkedList(LinkedList<T>&& obj) noexcept {
		this->head_ = obj.head_;
		this->tail_ = obj.tail_;
		this->len_ = obj.len_;
		obj.head_ = nullptr;
		obj.tail_ = nullptr;
	}

	template<typename T>
	void LinkedList<T>::insert(size_t index, T item) {
		if (index > this->len_)
			index = index % this->len_;
		if (this->len_ == 0) {
			this->head_ = new ListNode<T>(item);
			this->tail_ = this->head_;
			this->head_->next_ = this->head_;
			this->head_->prev_ = this->head_;
			this->len_ += 1;
			return;
		}
		if (index == 0) {
			auto old_head = this->head_;
			this->head_ = new ListNode<T>(item, this->tail_, old_head);
			this->tail_->next_ = this->head_;
			old_head->prev_ = this->head_;
		}
		else if (index == this->len_) {
			auto old_tail = this->tail_;
			this->tail_ = new ListNode<T>(item, old_tail, this->head_);
			this->head_->prev_ = this->tail_;
			old_tail->next_ = this->tail_;
		}
		else {
			ListNode<T>* next = this->head_;
			for (size_t i = 0; i < index; i++) {
				next = this->head_->next_;
			}
			ListNode<T>* prev = next->prev_;
			prev->next_ = new ListNode<T>(item, prev, next);
			next->prev_ = prev->next_;
		}
		this->len_ += 1;
	}

	template<typename T>
	std::optional<T> LinkedList<T>::remove(size_t index) {
		// TODO: 参考insert()添加操作头尾节点、判断无效index
		if (this->len_ == 0)
			return std::nullopt;
		if (index > this->len_) {
			index = index % this->len_;
		}
		ListNode<T>* node;
		if (index == 0) {
			node = this->head_;
			this->head_ = this->head_->next_;
			this->head_->prev_ = this->tail_;
			this->tail_->next_ = this->head_;
		}
		else if (index == this->len_ - 1) {
			node = this->tail_;
			this->tail_ = this->tail_->prev_;
			this->tail_->next_ = this->head_;
			this->head_->prev_ = this->tail_;
		}
		else {
			node = this->head_;
			for (size_t i = 0; i < index; i++) {
				node = node->next_;
			}
			node->prev_->next_ = node->next_;
			node->next_->prev_ = node->prev_;
		}
		this->len_ -= 1;
		T content = node->content;
		delete node;
		return { content };
	}

	template<typename T>
	LinkedList<T>::operator std::string()const {
		std::string s("[");
		ListNode<T>* n = this->head_;
		for (size_t i = 0; i < this->len_ - 1; i++) {
			s.append(fmt::format("{}, ", n->content));
			n = n->next_;
		}
		s.append(fmt::format("{}]", n->content));
		return s;
	}

	template<typename T>
	ListNode<T>::ListNode(T content) {
		this->content = content;
		this->next_ = nullptr;
		this->prev_ = nullptr;
	}

	template<typename T>
	ListNode<T>::ListNode(T content, ListNode<T>* prev, ListNode<T>* next) {
		this->content = content;
		this->prev_ = prev;
		this->next_ = next;
	}

	template<typename T>
	ListNode<T>::~ListNode() {
	}
}

#endif

