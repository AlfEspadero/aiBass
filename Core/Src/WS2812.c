/*
 * ARGB.c
 *
 *  Created on: Feb 18, 2025
 *      Author: Angel Jimenez
 */

#include "cmsis_os.h"
#include "tim.h"
#include "WS2812.h"
#include <math.h>


#define MAX_LED 8
//#define USE_BRIGHTNESS 1
#define WS2812_DMA_TIMEOUT_MS 20U

uint8_t LED_Data[MAX_LED][4];
uint8_t LED_Mod[MAX_LED][4];

void WS2812_Set_LED (int LEDnum, int Red, int Green, int Blue)
{
	if(LEDnum < MAX_LED){
		LED_Data[LEDnum][0] = LEDnum;
		LED_Data[LEDnum][1] = Green;
		LED_Data[LEDnum][2] = Red;
		LED_Data[LEDnum][3] = Blue;
	}
}

void WS2812_Clear_LED(){
	for(uint8_t i = 0; i < MAX_LED; i++){
		WS2812_Set_LED(i, 0, 0, 0);
	}
}

void WS2812_Set_All_LED( int Red, int Green, int Blue){
	for(uint8_t i = 0; i < MAX_LED; i++){
		WS2812_Set_LED(i, Red, Green, Blue);
	}
}

#define DEG_TO_RAD 0.01745329251994329576923690768489f	// = pi / 180


void WS2812_Set_Brightness (int brightness)  // 0-45
{
#if USE_BRIGHTNESS
	if (brightness > 45) brightness = 45;
	for (int i=0; i<MAX_LED; i++)
	{
		LED_Mod[i][0] = LED_Data[i][0];
		for (int j=1; j<4; j++)
		{
			float angle = 90-brightness;  // in degrees
			angle = angle * DEG_TO_RAD;  // in rad
			LED_Mod[i][j] = (LED_Data[i][j])/(tan(angle));
		}
	}
#endif
}

uint16_t pwmIdx = 0;
uint32_t pwmData[(24 * MAX_LED) + 50];
volatile uint8_t datasentflag = 0;

static osMutexId_t ws2812MutexHandle = NULL;
static const osMutexAttr_t ws2812Mutex_attributes = { .name = "ws2812Mutex" };
static osSemaphoreId_t ws2812DmaDoneSemHandle = NULL;
static const osSemaphoreAttr_t ws2812DmaDoneSem_attributes = { .name =
		"ws2812DmaDone" };

static void WS2812_RTOS_Init(void) {
	if (osKernelGetState() != osKernelRunning) {
		return;
	}

	if (ws2812MutexHandle == NULL) {
		ws2812MutexHandle = osMutexNew(&ws2812Mutex_attributes);
	}
	if (ws2812DmaDoneSemHandle == NULL) {
		ws2812DmaDoneSemHandle = osSemaphoreNew(1U, 0U,
				&ws2812DmaDoneSem_attributes);
	}
}

void WS2812_Update (void)
{
	uint32_t indx=0;
	uint32_t color;
	osKernelState_t kernel_state = osKernelGetState();
	uint8_t lock_acquired = 0U;

	if (kernel_state == osKernelRunning) {
		WS2812_RTOS_Init();
		if (ws2812MutexHandle != NULL) {
			if (osMutexAcquire(ws2812MutexHandle, osWaitForever) == osOK) {
				lock_acquired = 1U;
			} else {
				return;
			}
		}
	}

	for (int i= 0; i<MAX_LED; i++)
	{
#if USE_BRIGHTNESS
		color = ((LED_Mod[i][1]<<16) | (LED_Mod[i][2]<<8) | (LED_Mod[i][3]));
#else
		color = ((LED_Data[i][1]<<16) | (LED_Data[i][2]<<8) | (LED_Data[i][3]));
#endif

		for (int i=23; i>=0; i--)
		{
			if (color&(1<<i))
			{
				pwmData[indx] = 67;  // 2/3 of 100
			}

			else pwmData[indx] = 33;  // 1/3 of 100

			indx++;
		}
	}

	for (int i=0; i<50; i++)
	{
		pwmData[indx] = 0;
		indx++;
	}
	pwmIdx = 0;

	datasentflag = 0;
	if (ws2812DmaDoneSemHandle != NULL) {
		while (osSemaphoreAcquire(ws2812DmaDoneSemHandle, 0U) == osOK) {
		}
	}

	if (HAL_TIM_PWM_Start_DMA(&htim2, TIM_CHANNEL_1, pwmData,
			(24 * MAX_LED) + 50) == HAL_OK) {
		if ((kernel_state == osKernelRunning) && (ws2812DmaDoneSemHandle != NULL)) {
			if (osSemaphoreAcquire(ws2812DmaDoneSemHandle,
			WS2812_DMA_TIMEOUT_MS) != osOK) {
				(void) HAL_TIM_PWM_Stop_DMA(&htim2, TIM_CHANNEL_1);
			}
		} else {
			uint32_t start_tick = HAL_GetTick();
			while (datasentflag == 0U) {
				if ((HAL_GetTick() - start_tick) >= WS2812_DMA_TIMEOUT_MS) {
					(void) HAL_TIM_PWM_Stop_DMA(&htim2, TIM_CHANNEL_1);
					break;
				}
			}
		}
	}
	datasentflag = 0U;

	if (lock_acquired != 0U) {
		(void) osMutexRelease(ws2812MutexHandle);
	}
}



void HAL_TIM_PWM_PulseFinishedCallback(TIM_HandleTypeDef *htim)
{
	if ((htim == NULL) || (htim->Instance != TIM2)) {
		return;
	}

	(void) HAL_TIM_PWM_Stop_DMA(htim, TIM_CHANNEL_1);
	datasentflag = 1U;
	if ((osKernelGetState() == osKernelRunning)
			&& (ws2812DmaDoneSemHandle != NULL)) {
		(void) osSemaphoreRelease(ws2812DmaDoneSemHandle);
	}
}
