/*
 * LSM6DSL.c
 *
 *  Created on: Feb 7, 2025
 *      Author: Angel Jimenez
 */

#include "stm32l4xx_hal.h"
#include "LSM6DSL.h"

extern I2C_HandleTypeDef hi2c2;

#define LSM6DSL_I2C_TIMEOUT_MS 20U
#define LSM6DSL_STATUS_XLDA_MASK 0x01U

HAL_StatusTypeDef LSM6DSL_Init(void) {
	uint8_t value = ODR_XL_6660Hz;

	return HAL_I2C_Mem_Write(&hi2c2, LSM6DSL_ADDR, REG_CTRL1_XL,
	I2C_MEMADD_SIZE_8BIT, &value, 1, LSM6DSL_I2C_TIMEOUT_MS);
}

HAL_StatusTypeDef LSM6DSL_DataReady(uint8_t *ready) {
	uint8_t status = 0;
	HAL_StatusTypeDef ret;

	if (ready == NULL) {
		return HAL_ERROR;
	}

	ret = HAL_I2C_Mem_Read(&hi2c2, LSM6DSL_ADDR, REG_STATUS, I2C_MEMADD_SIZE_8BIT,
			&status, 1, LSM6DSL_I2C_TIMEOUT_MS);
	if (ret != HAL_OK) {
		*ready = 0;
		return ret;
	}

	*ready = ((status & LSM6DSL_STATUS_XLDA_MASK) != 0U) ? 1U : 0U;
	return HAL_OK;
}

HAL_StatusTypeDef LSM6DSL_WaitDataReady(uint32_t timeout_ms) {
	uint8_t ready = 0;
	uint32_t start = HAL_GetTick();
	HAL_StatusTypeDef ret;

	do {
		ret = LSM6DSL_DataReady(&ready);
		if (ret != HAL_OK) {
			return ret;
		}
		if (ready != 0U) {
			return HAL_OK;
		}
	} while ((HAL_GetTick() - start) < timeout_ms);

	return HAL_TIMEOUT;
}

HAL_StatusTypeDef LSM6DSL_ReadAccel(float accel[3]) {
	uint8_t buffer[6];
	HAL_StatusTypeDef ret;

	if (accel == NULL) {
		return HAL_ERROR;
	}

	ret = HAL_I2C_Mem_Read(&hi2c2, LSM6DSL_ADDR, REG_OUTX_L_XL, I2C_MEMADD_SIZE_8BIT,
			buffer, 6, LSM6DSL_I2C_TIMEOUT_MS);
	if (ret != HAL_OK) {
		accel[0] = 0.0f;
		accel[1] = 0.0f;
		accel[2] = 0.0f;
		return ret;
	}

	for (uint8_t i = 0; i < 3; i++) {
		int16_t raw = (int16_t) ((buffer[2 * i + 1] << 8) | buffer[2 * i]);
		accel[i] = ((float) raw) * 0.061f;
	}

	return HAL_OK;
}
