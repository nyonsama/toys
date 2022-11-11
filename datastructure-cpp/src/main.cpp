#include<vector>
#include<string>
#include<iostream>
#include<fstream>
#include<fmt/core.h>
#include"linked_list.h"
#include"lab1/joseph_ring.h"
#include"lab2/brackets_match.h"
#include"lab2/parking_lot.h"
#include"lab2/maze.h"
#include"lab3/median.h"
#include"lab4/change.h"

using namespace std;
using linked_list::LinkedList;
using fmt::print;
void test_memory_leak();
void test_linked_list();
void test_joseph_ring();
void test_parking_lot();
void test_brackets_match();
void test_maze();
void test_median();
void test_change();
int main()
{
	// test_memory_leak();
	// test_linked_list();
	bool flag = true;
	while (flag && !cin.eof()) {
		print("choose a lab to run.\n");
		print("1. lab1::joseph_ring\n");
		print("2. lab2::parking_lot\n");
		print("3. lab2::brackets_match\n");
		print("4. lab2::maze\n");
		print("5. lab3::midian\n");
		print("6. lab4::change\n");
		print("q. exit\n");
		string choice;
		cin >> choice;
		switch (choice[0]) {
		case '1': test_joseph_ring(); break;
		case '2': test_parking_lot(); break;
		case '3': test_brackets_match(); break;
		case '4': test_maze(); break;
		case '5': test_median(); break;
		case '6': test_change(); break;
		case 'q': flag = false; break;
		default: break;
		}
	}
	return 0;
}

void test_memory_leak() {
	for (size_t i = 0; i < 10000; i++) {
		LinkedList<size_t> asdf;
		for (size_t j = 0; j < 100; j++) {
			// asdf.push_back(make_shared<size_t>(9));
			asdf.push_back(9);
		}
	}
}

void test_linked_list() {
	LinkedList<size_t> crap;
	crap.push_back(0);
	crap.push_back(1);
	crap.push_back(2);
	crap.push_back(3);
	crap.push_back(4);
	crap.push_back(5);
	crap.push_back(6);
	auto iter = crap.iter();
	print("{}\n", iter->next());
	print("{}\n", iter->next());
	print("{}\n", iter->next());
	print("{}\n", iter->last());
}

void test_joseph_ring() {
	print("{:-^60}\n", "joseph ring");
	auto passwords = vector<size_t>{ 3, 1, 7, 2, 4, 8, 4 };
	lab1::joseph_ring::JosephRing jr(7, 20, passwords);
	print("{}\n\n", string(jr.run()));
}

void test_parking_lot() {
	print("{:-^60}\n", "parking lot");
	print("{}\n", "please input parking lot data. input `e` and hit enter to run a example.");
	string data;
	cin >> data;
	if (data == "e")
		data = "('A',1,5),('A',2,10),('D',1,15),('A',3,20),('A',4,25),('A',5,30),('D',2,35),('D',4,40),('E',0,0)";
	print("data:{}\n", data);
	lab2::parking_lot::ParkingLot::run(2, data);
	std::puts("");
}

void test_brackets_match() {
	using lab2::brackets_match::brackets_match;
	print("{:-^60}\n", "brackets match");
	print("{}\n", "please input a string. input `e` and hit enter to run some examples.");
	string str;
	cin >> str;
	if (str == "e") {
		vector<string> examples{ "a+(2+[2+2])", "a+[2+(2+2)]", ")(1+2)","[8*9+(10+10)" };
		for (string s : examples) {
			print("{} {}\n", s, brackets_match(s));
		}
	}
	else
		print("{} {}\n", str, brackets_match(str));
	std::puts("");
}

void test_maze() {
	print("{:-^60}\n", "maze");
	auto m = lab2::maze::Maze(
		8, // len
		9, // height
		vector<vector<uint8_t>>{
			vector<uint8_t>{ 0, 0, 1, 0, 0, 0, 1, 0 },
			vector<uint8_t>{ 0, 0, 1, 0, 0, 0, 1, 0 },
			vector<uint8_t>{ 0, 0, 0, 0, 1, 1, 0, 1 },
			vector<uint8_t>{ 0, 1, 1, 1, 0, 0, 1, 0 },
			vector<uint8_t>{ 0, 0, 0, 1, 0, 0, 0, 0 },
			vector<uint8_t>{ 0, 1, 0, 0, 0, 1, 0, 1 },
			vector<uint8_t>{ 0, 1, 1, 1, 1, 0, 0, 1 },
			vector<uint8_t>{ 1, 1, 0, 0, 0, 1, 0, 1 },
			vector<uint8_t>{ 1, 1, 1, 0, 0, 0, 0, 0 },
		},
		// (row, column)
		lab2::maze::Position(1, 1),
		lab2::maze::Position(9, 8)
	);
	auto path = m.run();
	print("(row, column, direction)\n");
	print("{}\n\n", string(path));
}

void test_median() {
	using lab3::median::get_median;
	print("{:-^60}\n", "median");
	print("{}\n", "input 'e' to run a example, 'f' to process input.txt.");
	char choice;
	double result;
	cin >> choice;
	if (choice == 'e') {
		vector<double>x = { 5,15,18 };
		vector<double>y = { 3,14,21 };
		result = get_median(x, y);
	}
	else if (choice == 'f') {
		vector<double>x, y;
		ifstream ifs("input.txt");
		ofstream ofs("output.txt");
		size_t width;
		ifs >> width;
		double d;
		for (size_t i = 0; i < width; i++) {
			ifs >> d;
			x.push_back(d);
		}
		for (size_t i = 0; i < width; i++) {
			ifs >> d;
			y.push_back(d);
		}
		result = get_median(x, y);
		ifs.close();
		ofs << result;
		ofs.close();
	}
	else 
		return;
	print("result:{}\n\n", result);
}

void test_change() {
	using lab4::change::change;
	print("{:-^60}\n", "change");
	print("{}\n", "input 'e' to run a example, 'f' to process input.txt.");
	// auto n = lab4::change::change(9, vector<double>{1, 2, 5});
	// auto n = lab4::change::change(15, vector<double>{1, 5, 11});

	char choice;
	size_t result;
	cin >> choice;
	if (choice == 'e') {
		result = change(9, vector<double>{1, 2, 5});
	}
	else if (choice == 'f') {
		ifstream ifs("input.txt");
		ofstream ofs("output.txt");
		vector<double> cashes;
		size_t len;
		ifs >> len;
		double d;
		for (size_t i = 0; i < len; i++) {
			ifs >> d;
			cashes.push_back(d);
		}
		ifs >> d;
		result = change(d, cashes);
		ifs.close();
		ofs << result;
		ofs.close();
	}
	else 
		return;
	
	print("result:{}\n\n", result);
}

