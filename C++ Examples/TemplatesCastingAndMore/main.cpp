#include<iostream>

namespace MyNamespace {
	template<class T>
	struct MyStruct {
	private:
		T var;
	public:
		MyStruct(const T& var) : var(var) {}
		void setVar(const T& newVar) {
			var = newVar;
		}

		T getVar() const {
			return var;
		}

		void print() const {
			std::cout << "Value: " << var << std::endl;
		}

		friend void swap(MyStruct<T>& lhs, MyStruct<T>& rhs) {
			using std::swap;
			swap(lhs.var, rhs.var);
		}
	};

	template<class T>
	const MyStruct<T> operator+(const MyStruct<T>& lhs, const MyStruct<T>& rhs) {
		return MyStruct<T>(lhs.getVar() + rhs.getVar());
	}
}

class MyClass {
	int a;

	void procedure() {
		//calcs
		a = 8;
	}
public:
	MyClass(int a = 0) : a(a) {}

	const int& get() const {
		return a;
	}
};

int main() {
	MyNamespace::MyStruct<float> x(5), y(8);
	auto z = x + y;
	z.print();
	std::swap(x, y);
	x.print();
	y.print();
	MyClass a(2);
	const int& Consant = a.get();
	int& NonConst = const_cast<int&>(Consant);
	NonConst = 333;
	std::cout << a.get() << std::endl;
	return 0;
}
