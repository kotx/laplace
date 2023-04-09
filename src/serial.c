#include <stdio.h>

int serial_send(double* accel, double* gyro)
{
	printf("a: %.2f %.2f %.2f\n", accel[0], accel[1], accel[2]);
	printf("g: %.2f %.2f %.2f\n", gyro[0], gyro[1], gyro[2]);
}
