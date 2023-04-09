#include "hardware/i2c.h"
#include "mpu6050.h"
#include "pico/stdlib.h"
#include <stdio.h>
#include <string.h>

#include "serial.h"

const int POLLING_RATE_HZ = 50;

int main(void)
{
	gpio_init(PICO_DEFAULT_LED_PIN);
	gpio_set_dir(PICO_DEFAULT_LED_PIN, 1);

	stdio_init_all();

#if STDIO_USB_WAIT
	while (!stdio_usb_connected()) {
		gpio_put(PICO_DEFAULT_LED_PIN, 1);
		sleep_ms(100);
		gpio_put(PICO_DEFAULT_LED_PIN, 0);
		sleep_ms(100);
	}
#endif

	printf("initializing mpu6050...");
	int err = mpu6050_init();
	if (err != 0) {
		printf("error: %i\n", err);
		return err;
	}
	printf("done\n");

	int16_t accel[3], gyro[3], temp;

	// absolute_time_t ts = get_absolute_time();

	while (true) {
		// int64_t dt = absolute_time_diff_us(ts, get_absolute_time());
		mpu6050_read_raw(accel, gyro);

		double accel_d[3], gyro_d[3];
		for (int i = 0; i < 3; i++)
			accel_d[i] = (double)accel[i];
		for (int i = 0; i < 3; i++)
			gyro_d[i] = (double)gyro[i];

		mpu6050_normalize_raw(accel_d, gyro_d);

		// TODO: subtract gravity from accelerometer value
		serial_send(accel_d, gyro_d);

		// ts = get_absolute_time();
		sleep_ms(1000 / POLLING_RATE_HZ);
	}
	return 0;
}
