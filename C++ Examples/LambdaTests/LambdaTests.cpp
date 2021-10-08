#include<iostream>
#include<set>
#include<vector>
#include<algorithm>
#include <fstream>
#include<memory>
using namespace std;

auto print(int a) -> void {
	std::cout << a << ' ';
}

#define read_file std::ifstream file; \
file.open("../../../numbers.txt",std::ios::in); \
if (!file.is_open())\
	return 1;\
short i; \
std::vector<short> numbers;\
while (file >> i) {\
	numbers.push_back(i);\
}\
file.close();


struct Rational {
	Rational(int a = 0, int b = 0) : a(a), b(b) {}
	int a;
	int b;
private:
	mutable std::size_t m_length;

public:

	void print() const {
		std::cout << a << ' ' << b << std::endl;
	}

	std::size_t length() const {
		m_length = a + b;
		return m_length;
	}
};

const Rational operator*(const Rational& lhs, const Rational& rhs) {
	return Rational(lhs.a * rhs.a, lhs.b * rhs.b);
}

class A {
public:
	A(int a = 0) {
		data = new int;
		(*data) = a;
	}
	~A() {
		delete data;
	}
	A(const A& rhs) {
		data = new int;
		(*data) = (*rhs.data);
	}
	A(A&& rhs) noexcept {
		swap(*this, rhs);
	}
	A& operator=(A rhs) {
		swap(*this, rhs);
		return *this;
	}

	A& operator=(A&& rhs) noexcept {
		swap(*this, rhs);
		rhs.data = nullptr;
		return *this;
	}

	int getData() const {
		return (*data);
	}

	void setData(int newData) {
		(*data) = newData;
	}

	void print() const {
		std::cout << "Data = " << (*data) << "\nAt address = " << data << '\n';
	}

private:
	static void swap(A& lhs, A& rhs) {
		std::swap(lhs.data, rhs.data);
	}

	int* data = nullptr;
};

struct Base {
	Base(int a = 0) : a(a) {}
	virtual ~Base() {}
	virtual void printType() const = 0;
	int a = 0;
};

struct Child1 : public Base {
	Child1(int a = 1, std::string type = "Child1") : Base(a), type(type) {}

	virtual void printType() const override {
		std::cout << "Child1 type" << std::endl;
		std::cout << a << ' ' << type << std::endl;
	}


	std::string type = "Child1";
};

struct Child2 : public Base {
	Child2(int a = 2, std::string type = "Child2") : Base(a), type(type) {}

	virtual void printType() const override {
		std::cout << "Child2 type" << std::endl;
		std::cout << a << ' ' << type << std::endl;
	}


	std::string type = "Child2";
};

inline int f(double a, float b) {
	return (int)a * b;
}

int execute(int a, int (*fptr) (int, int)) {
	return a * fptr(2, 3);
}

int main() {
	Rational a(1, 2), b(3, 2);
	Rational result = a * b;
	result.print();

	std::vector<short> v({ 11,22,33 });
	std::vector<short>::iterator i0 = v.begin();
	std::vector<short>::const_iterator i1 = v.begin();
	const std::vector<short>::iterator i2 = v.begin();
	const std::vector<short>::const_iterator i3 = v.begin();

	(*i0) = 2;
	std::cout << (*i0) << (*++i0) << std::endl;

	//(*i1) = 2; error
	std::cout << (*i1) << (*++i1) << std::endl;

	(*i2) = 2;
	std::cout << (*i2) << /*(*++i2) error <<*/ std::endl;

	//(*i3) = 2; error
	std::cout << (*i3) << /*(*++i3) error << */ std::endl;

	std::unique_ptr<A> ptr = std::make_unique<A>(5);
	ptr->print();

	A secondA;
	secondA = (*ptr);

	std::shared_ptr<A> ptr2 = std::make_shared<A>(8);
	{
		auto ptr3 = ptr2;
	}

	std::shared_ptr<Base> ptrToBase = std::make_shared<Child2>();
	ptrToBase->printType();
	ptrToBase = std::make_shared<Child1>();
	ptrToBase->printType();

	std::vector<std::unique_ptr<Base>> vectorOfBase;
	vectorOfBase.push_back(std::make_unique <Child1>(100, "Petko"));
	vectorOfBase.push_back(std::make_unique<Child2>(200, "Krasi"));
	for (const auto& child : vectorOfBase) {
		child->printType();
	}

	auto fptr = [](int d, int f) -> int {
		return 2 * d + 3 * f * 2 * d + 3 * f;
	};

	auto fptr1 = [&](int d, int f) -> int {
		return a.a * d + 3 * f * 2 * d + b.a * f;
	};

	std::cout << execute(3, fptr) << std::endl;

	std::cout << fptr1(4, 6) << std::endl;

	/*std::vector<int> s;
	constexpr int n = 10;
	for (int i = 0; i <= n; ++i)
		s.push_back(i);

	auto increase = [](int& p) -> void { ++p; };
	constexpr auto sumOfNumbersToN = [](unsigned int to) -> unsigned int {
		unsigned int result = 0;
		for (unsigned int i = 1; i <= to; ++i)
			result += i;
		return result;
	};

	std::for_each(s.begin(), s.end(), increase);
	for (const auto& i : s)
		print(i);
	cout << endl;
	cout << sumOfNumbersToN(10) << endl;

	read_file;
	for (const short& i : numbers)
		std::cout << i << std::endl;*/
	return 0;
}
