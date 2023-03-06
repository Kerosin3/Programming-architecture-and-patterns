cmake_minimum_required(VERSION 3.14)
project(mainapp LANGUAGES CXX)

ADD_EXECUTABLE ( ${PROJECT_NAME}
		main.cpp)
TARGET_LINK_LIBRARIES( ${PROJECT_NAME}
			gtest
			gmock
	      		)
