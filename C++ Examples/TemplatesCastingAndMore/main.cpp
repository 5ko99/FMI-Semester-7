#include<iostream>
#include<list>

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

//NVI idiom
class Character {
protected:
	int health = 10;
private:

	virtual int doHealthValue() const {
		return health + 8;
	}

public:
	int healthValue() const {
		int retValue = doHealthValue();
		return retValue;
	}
};

class ConcreteCharacter : public Character {
private:
	virtual int doHealthValue() const override {
		return health + 58;
	}
};

template<class T>
class Set {
public:
	void insert(const T& element) {
		auto ptr = std::find(list.begin(), list.end(), element);
		if (ptr == list.end()) {
			list.push_back(element);
		}
	}

	std::size_t size() const {
		return list.size();
	}

	bool contains(const T& element) const {
		return std::find(list.begin(), list.end(), element) != list.end() ? true : false;
	}

	void remove(const T& element) {
		list.remove(element);
	}
private:
	std::list<T> list;
};

template<class T>
class SometingThatMustUseSet : private Set<T> {
public:
	void trackAndStart(const T& element) {
		Set<T>::insert(element);
		std::cout << "Started: " << element << std::endl;
	}

	void stop(const T& element) {
		if (Set<T>::contains(element)) {
			Set<T>::remove(element);
			std::cout << "Stopped: " << element << std::endl;
		}
		else {
			throw 0;
		}
	}
};

//using of typename
struct typedefStruct {
	typedef int* ptr;
};


template <class T>
struct S {
	typename T::ptr i;
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
	Character* c = new ConcreteCharacter();
	std::cout << c->healthValue() << std::endl;
	auto f1 = [](int x, int y) { return x + y; };
	Set<int> s;
	s.insert(2);
	s.insert(8);
	std::cout << s.contains(8) << std::endl;
	SometingThatMustUseSet<int> something;
	something.trackAndStart(2);
	something.trackAndStart(3);
	something.stop(3);
	S<typedefStruct> var;
	int number = 3;
	var.i = &number;
	return 0;
}
