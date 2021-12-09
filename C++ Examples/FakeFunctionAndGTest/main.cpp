#include<iostream>
#include"fff.h"

DEFINE_FFF_GLOBALS;
FAKE_VOID_FUNC(DISPLAY_init);

FAKE_VOID_FUNC(DISPLAY_output, char*);

FAKE_VALUE_FUNC(double, pow1, double, double);



FAKE_VOID_FUNC(voidfunc2, char, char);
FAKE_VALUE_FUNC(long, longfunc0);
int main() {
	DISPLAY_init();
	DISPLAY_init();
	std::cout << DISPLAY_init_fake.call_count << std::endl;

	char msg[] = "helloworld";
	DISPLAY_output(msg);
	std::cout << (DISPLAY_output_fake.call_count, 1) << '\n' << DISPLAY_output_fake.arg0_val << std::endl << std::endl;

	pow1_fake.return_val = 8;
	double result = pow1(2, 3);
	std::cout << result << std::endl;

	RESET_FAKE(pow1);

	FFF_RESET_HISTORY();

	longfunc0();
	voidfunc2('a', 'b');
	longfunc0();

	std::cout << fff.call_history[0] << '-' << (void*)longfunc0 << std::endl;


	return 0;
}

