/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * File Name          : freertos.c
  * Description        : Code for freertos applications
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2026 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */

/* Includes ------------------------------------------------------------------*/
#include "FreeRTOS.h"
#include "task.h"
#include "main.h"
#include "cmsis_os.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

#include <stdio.h>
#include "LSM6DSL.h"
#include "NanoEdgeAI.h"
#include "WS2812.h"

/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN PTD */

/* USER CODE END PTD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN PD */

#define WINDOW_SIZE 64
#define AXES 3
#define WINDOW_READY_FLAG (1U << 0)

static float window[WINDOW_SIZE * AXES];

/* USER CODE END PD */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN PM */

float input_user_buffer[NEAI_INPUT_SIGNAL_LENGTH * NEAI_INPUT_AXIS_NUMBER];
float probabilities[NEAI_NUMBER_OF_CLASSES]; // Buffer of class probabilities

void acquire_window(void) {
	for (int i = 0; i < WINDOW_SIZE; i++) {
		if (LSM6DSL_WaitDataReady(10U) != HAL_OK) {
			window[3 * i] = 0.0f;
			window[3 * i + 1] = 0.0f;
			window[3 * i + 2] = 0.0f;
			continue;
		}
		(void) LSM6DSL_ReadAccel(&window[3 * i]);
	}
}

void transmit_window(void) {
	for (int i = 0; i < WINDOW_SIZE; i++) {
		printf("%f %f %f ", window[3 * i], window[3 * i + 1],
				window[3 * i + 2]);
	}
	printf("\r\n");
}

/* USER CODE END PM */

/* Private variables ---------------------------------------------------------*/
/* USER CODE BEGIN Variables */

/* USER CODE END Variables */
/* Definitions for defaultTask */
osThreadId_t defaultTaskHandle;
const osThreadAttr_t defaultTask_attributes = {
  .name = "defaultTask",
  .stack_size = 256 * 4,
  .priority = (osPriority_t) osPriorityNormal,
};
/* Definitions for neai_classify */
osThreadId_t neai_classifyHandle;
const osThreadAttr_t neai_classify_attributes = {
  .name = "neai_classify",
  .stack_size = 256 * 4,
  .priority = (osPriority_t) osPriorityNormal,
};
/* Definitions for acquire_window */
osThreadId_t acquire_windowHandle;
const osThreadAttr_t acquire_window_attributes = {
  .name = "acquire_window",
  .stack_size = 256 * 4,
  .priority = (osPriority_t) osPriorityNormal,
};
/* Definitions for ledtask */
osThreadId_t ledtaskHandle;
const osThreadAttr_t ledtask_attributes = {
  .name = "ledtask",
  .stack_size = 256 * 4,
  .priority = (osPriority_t) osPriorityAboveNormal,
};
/* Definitions for windowMutex */
osMutexId_t windowMutexHandle;
const osMutexAttr_t windowMutex_attributes = {
  .name = "windowMutex"
};
/* Definitions for uartTxMutex */
osMutexId_t uartTxMutexHandle;
const osMutexAttr_t uartTxMutex_attributes = {
  .name = "uartTxMutex"
};

/* Private function prototypes -----------------------------------------------*/
/* USER CODE BEGIN FunctionPrototypes */

/* USER CODE END FunctionPrototypes */

void StartDefaultTask(void *argument);
void t_neai_classify(void *argument);
void t_acquire_window(void *argument);
void t_ledtask(void *argument);

void MX_FREERTOS_Init(void); /* (MISRA C 2004 rule 8.1) */

/**
  * @brief  FreeRTOS initialization
  * @param  None
  * @retval None
  */
void MX_FREERTOS_Init(void) {
  /* USER CODE BEGIN Init */

  /* USER CODE END Init */
  /* Create the mutex(es) */
  /* creation of windowMutex */
  windowMutexHandle = osMutexNew(&windowMutex_attributes);

  /* creation of uartTxMutex */
  uartTxMutexHandle = osMutexNew(&uartTxMutex_attributes);

  /* USER CODE BEGIN RTOS_MUTEX */
  /* add mutexes, ... */
  /* USER CODE END RTOS_MUTEX */

  /* USER CODE BEGIN RTOS_SEMAPHORES */
  /* add semaphores, ... */
  /* USER CODE END RTOS_SEMAPHORES */

  /* USER CODE BEGIN RTOS_TIMERS */
  /* start timers, add new ones, ... */
  /* USER CODE END RTOS_TIMERS */

  /* USER CODE BEGIN RTOS_QUEUES */
  /* add queues, ... */
  /* USER CODE END RTOS_QUEUES */

  /* Create the thread(s) */
  /* creation of defaultTask */
  defaultTaskHandle = osThreadNew(StartDefaultTask, NULL, &defaultTask_attributes);

  /* creation of neai_classify */
  neai_classifyHandle = osThreadNew(t_neai_classify, NULL, &neai_classify_attributes);

  /* creation of acquire_window */
  acquire_windowHandle = osThreadNew(t_acquire_window, NULL, &acquire_window_attributes);

  /* creation of ledtask */
  ledtaskHandle = osThreadNew(t_ledtask, NULL, &ledtask_attributes);

  /* USER CODE BEGIN RTOS_THREADS */
  /* add threads, ... */
  if ((defaultTaskHandle == NULL) || (neai_classifyHandle == NULL)
  		|| (acquire_windowHandle == NULL) || (ledtaskHandle == NULL)) {
  	Error_Handler();
  }
  /* USER CODE END RTOS_THREADS */

  /* USER CODE BEGIN RTOS_EVENTS */
  /* add events, ... */
  /* USER CODE END RTOS_EVENTS */

}

/* USER CODE BEGIN Header_StartDefaultTask */
/**
  * @brief  Function implementing the defaultTask thread.
  * @param  argument: Not used
  * @retval None
  */
/* USER CODE END Header_StartDefaultTask */
void StartDefaultTask(void *argument)
{
  /* USER CODE BEGIN StartDefaultTask */
  /* Infinite loop */
  for(;;)
  {
    osDelay(1);
  }
  /* USER CODE END StartDefaultTask */
}

/* USER CODE BEGIN Header_t_neai_classify */
/**
 * @brief Function implementing the neai_classify thread.
 * @param argument: Not used
 * @retval None
 */
/* USER CODE END Header_t_neai_classify */
void t_neai_classify(void *argument)
{
  /* USER CODE BEGIN t_neai_classify */
	int id_class = 0;
	/* Infinite loop */
	for (;;) {
		osThreadFlagsWait(WINDOW_READY_FLAG, osFlagsWaitAny, osWaitForever);

		if (windowMutexHandle != NULL) {
			osMutexAcquire(windowMutexHandle, osWaitForever);
		}
		neai_classification(window, probabilities, &id_class);
		if (windowMutexHandle != NULL) {
			osMutexRelease(windowMutexHandle);
		}

		if (neai_get_class_name(id_class)[0] == 'N') {
			printf("\033[2J");  // Clear the screen
			printf("\033[H");   // Move cursor to home position
			printf("NOISE\r\n");
		} else {
			printf("%s\r\n", neai_get_class_name(id_class));
		}

	}
  /* USER CODE END t_neai_classify */
}

/* USER CODE BEGIN Header_t_acquire_window */
/**
 * @brief Function implementing the acquire_window thread.
 * @param argument: Not used
 * @retval None
 */
/* USER CODE END Header_t_acquire_window */
void t_acquire_window(void *argument)
{
  /* USER CODE BEGIN t_acquire_window */
	/* Infinite loop */
	for (;;) {
		if (windowMutexHandle != NULL) {
			osMutexAcquire(windowMutexHandle, osWaitForever);
		}
		acquire_window();
		if (windowMutexHandle != NULL) {
			osMutexRelease(windowMutexHandle);
		}

		if (neai_classifyHandle != NULL) {
			osThreadFlagsSet(neai_classifyHandle, WINDOW_READY_FLAG);
		}

		osDelay(1);
	}
  /* USER CODE END t_acquire_window */
}

/* USER CODE BEGIN Header_t_ledtask */
/**
* @brief Function implementing the ledtask thread.
* @param argument: Not used
* @retval None
*/
/* USER CODE END Header_t_ledtask */
void t_ledtask(void *argument) {
	/* USER CODE BEGIN t_ledtask */
	/* Infinite loop */
	for (;;) {
		for (int i = 0; i < 3; i++) {
			switch (i) {
			case 0:
				WS2812_Set_All_LED(255, 0, 0);
				break;
			case 1:
				WS2812_Set_All_LED(0, 255, 0);
				break;
			case 2:
				WS2812_Set_All_LED(0, 0, 255);
				break;
			default:
				WS2812_Clear_LED();
				break;
			}
			WS2812_Update();
			osDelay(1000);
		}
	}
	/* USER CODE END t_ledtask */
}

/* Private application code --------------------------------------------------*/
/* USER CODE BEGIN Application */

/* USER CODE END Application */

