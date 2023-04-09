set(EXE_NAME laplace)
add_executable(${EXE_NAME} main.c mpu6050.c serial.c)

pico_set_program_name(${EXE_NAME} "${EXE_NAME}")
pico_set_program_version(${EXE_NAME} "0.1")

pico_enable_stdio_uart(${EXE_NAME} 1)
pico_enable_stdio_usb(${EXE_NAME} 1)

target_link_libraries(${EXE_NAME} pico_stdlib)
target_include_directories(${EXE_NAME} PRIVATE ${CMAKE_CURRENT_LIST_DIR})

target_compile_definitions(${EXE_NAME} PUBLIC
	STDIO_USB_WAIT=0
)

target_link_libraries(${EXE_NAME} hardware_i2c)

pico_add_extra_outputs(${EXE_NAME})
