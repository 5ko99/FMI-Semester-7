#include "pch.h"
#include<queue>
#include"lib/fff/fff.h"
#include<string>

TEST(TestCaseName, TestName) {
	EXPECT_EQ(1, 1);
	EXPECT_TRUE(true);
}

TEST(Factoriel, HandleZeroInput) {
	EXPECT_EQ(factoriel(0), 1);
}

FAKE_VOID_FUNC(print, std::string);

class QueueTests : public ::testing::Test {
protected:
	void SetUp() override {
		q.push(1);
		q.push(2);
		q.push(3);
	}

	void TearDown() override {
		q.pop(); q.pop(); q.pop();
		assert(q.empty());
	}
	std::queue<int> q;
};

TEST_F(QueueTests, IsEmptyInitially) {
	EXPECT_EQ(q.size(), 3);
}

//TEST(PrintTest, canPrint) {
//	print("wwe");
//	ASSERT_EQ(print_fake.call_count, 1);
//	ASSERT_EQ(print_fake.arg0_val, std::string("petko"));
//}