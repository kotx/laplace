#pragma once

int mpu6050_init();
void mpu6050_read_raw(int16_t accel[3], int16_t gyro[3]);
void mpu6050_normalize_raw(double accel[3], double gyro[3]);
