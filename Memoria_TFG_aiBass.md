# Memoria TFG — aiBass (Borrador base editable)

> **Importante**  
> Este documento está redactado para que puedas pasarlo a Google Docs y completar los huecos.  
> **No se inventan datos**: todo lo no confirmado queda marcado como `[[PENDIENTE]]` o `[[SUSTITUIR]]`.

---

## Distribución objetivo de páginas (aprox. 60 en total)

| Apartado | Páginas objetivo |
|---|---:|
| 1. Portada | 1 |
| 2. Resumen | 1 |
| 3. Abstract | 1 |
| 4. Índice | 1 |
| 5. Índice de figuras | 1 |
| 6. Objetivos | 2 |
| 7. Introducción | 4 |
| 8. Estado del arte | 8 |
| 9. Tecnologías empleadas | 6 |
| 10. Arquitectura del sistema | 4 |
| 11. Desarrollo | 16 |
| 12. Pruebas del sistema | 6 |
| 13. Planificación temporal | 3 |
| 14. Costes | 3 |
| 15. Conclusiones | 1 |
| 16. Trabajo futuro | 1 |
| 17. Bibliografía | 1 |
| 18. Anexos | 1 |
| **TOTAL** | **60** |

### Control rápido de cumplimiento de plantilla oficial

- **Resumen**: redactado para mantenerse en 1 página.
- **Resumen en inglés**: incluido.
- **Objetivos**: separados en profesional y educacional.
- **Introducción**: preparada para 4-5 páginas.
- **Estado del arte**: estructurado por soluciones comparables con pros/contras.
- **Tecnologías**: incluye hardware, software y espacio para fotografías/datasheets.
- **Arquitectura**: incluye diagrama global y relación tecnología-bloque.
- **Desarrollo**: incluye funcionamiento, problemas, alternativas desechadas e integración.
- **Pruebas**: plan formal de pruebas por casos.
- **Planificación temporal**: tabla de hitos + Gantt + análisis de tiempos.
- **Costes**: material, personal (COCOMO u otra métrica), prototipo y producción.
- **Conclusiones, trabajo futuro, bibliografía y anexos**: incluidos.

---

## 1. Portada

`[[USAR PORTADA OFICIAL DE LA ESCUELA]]`

- Título del TFG: **aiBass**
- Autor: **Alfonso Espadero García**
- Curso: **2025/2026**
- Tutor: **Ángel Jiménez Fernández**
- Cotutor: **Daniel Casanueva Morato**
- Titulación: **Ingeniería Informática. Ingeniería de Computadores**
- Universidad/Centro: **Universidad de Sevilla**

---

## 2. Resumen

El proyecto **aiBass** plantea un sistema de inteligencia artificial embebida orientado a la detección de notas musicales en bajo eléctrico a partir de información inercial. La idea principal es aprovechar las vibraciones asociadas a la ejecución del instrumento, capturarlas mediante una IMU y procesarlas en una plataforma embebida para estimar, en tiempo real, la nota que está sonando.

El enfoque del trabajo combina instrumentación física, adquisición de datos, tratamiento de señal e inferencia de un modelo de clasificación en un entorno con restricciones de recursos. Esto sitúa el proyecto en la intersección entre sistemas empotrados, aprendizaje automático aplicado y tecnología musical.

En el estado actual de desarrollo se ha logrado detectar cinco clases de salida: las cuatro notas fundamentales del bajo (**E, A, D, G**) y una clase de **ruido/silencio** (ausencia de nota válida), mostrando el resultado por interfaz serie. Durante el desarrollo se evaluaron diferentes ubicaciones del sensor: una primera etapa con colocación en el mástil ofreció baja fiabilidad, por lo que el prototipado evolucionó hacia una configuración de prueba con el sensor sobre el amplificador.

Una parte relevante del trabajo se dedicó a estudiar la viabilidad de un enfoque alternativo de firmware en **Rust con Embassy**. Esta línea consumió aproximadamente **100 horas** y finalmente se descartó por no aportar, en este momento del proyecto, una relación coste/beneficio favorable respecto al objetivo de completar un prototipo funcional. El desarrollo total considerado en la memoria se sitúa en torno a **300 horas**.

El documento describe la motivación del problema, el estado del arte, las tecnologías utilizadas, la arquitectura general, el desarrollo técnico por bloques, el plan de pruebas y la planificación del proyecto, incluyendo una estimación de costes. Finalmente, se recogen conclusiones, limitaciones actuales y posibles líneas de trabajo futuro para mejorar robustez, generalización y aplicabilidad práctica.

---

## 3. Abstract

The **aiBass** project proposes an embedded artificial intelligence system for musical note detection in electric bass using inertial data. The main idea is to exploit instrument vibrations, capture them through an IMU, and process them on an embedded platform to estimate, in real time, which note is being played.

The project combines hardware instrumentation, data acquisition, signal processing, and embedded inference under resource constraints. Therefore, it lies at the intersection of embedded systems, applied machine learning, and music technology.

At the current development stage, the system can detect five output classes: the four fundamental bass notes (**E, A, D, G**) and a **noise/silence** class (no valid note sounding), with results printed through a serial interface. During development, different sensor locations were tested: an initial neck-mounted setup provided low reliability, so the prototype evolved to a test setup with the sensor placed on top of the amplifier.

A significant development phase focused on assessing a firmware approach in **Rust with Embassy**. This line of work took approximately **100 hours** and was eventually dropped, as it did not provide the best cost/benefit ratio for delivering the current prototype scope. The total effort considered in this report is approximately **300 hours**.

This report presents the project motivation, state of the art, employed technologies, overall architecture, technical development by modules, testing approach, and project planning, including cost estimation. Finally, conclusions, current limitations, and future work are discussed to improve robustness, generalization, and practical applicability.

---

## 4. Índice

`[[GENERAR AUTOMÁTICAMENTE EN GOOGLE DOCS / WORD]]`

---

## 5. Índice de Figuras

`[[GENERAR AUTOMÁTICAMENTE EN GOOGLE DOCS / WORD]]`

---

## 6. Objetivos del proyecto

### 6.1 Objetivos técnicos/profesionales

1. Diseñar una solución de detección de notas en bajo basada en señales inerciales.
2. Implementar una cadena completa desde captura de datos hasta inferencia embebida.
3. Construir y gestionar un conjunto de datos representativo de las clases objetivo.
4. Desarrollar un clasificador capaz de distinguir **E, A, D, G** y **ruido/silencio**.
5. Integrar una salida de resultado en tiempo real mediante interfaz serie.
6. Analizar el comportamiento del sistema ante cambios de montaje y condiciones de medida.

### 6.2 Objetivos formativos/educacionales

1. Profundizar en sistemas embebidos orientados a IA en el borde (edge AI).
2. Aprender metodologías de adquisición de datos para clasificación supervisada.
3. Consolidar competencias de validación experimental y análisis de resultados.
4. Practicar integración HW/SW en un contexto realista de prototipado.
5. Mejorar la capacidad de toma de decisiones técnicas ante resultados no ideales.
6. Evaluar y descartar de forma justificada alternativas tecnológicas cuando no encajan en alcance/plazo (caso Rust + Embassy).

### 6.3 Relación con objetivos inicialmente planteados

| Objetivo inicial | Estado | Evidencia |
|---|---|---|
| Estudiar posicionamiento del sensor e interfaz FW/SW | En progreso | Pruebas en mástil y transición a prueba en amplificador |
| Construir dataset de muestras | `[[PENDIENTE DETALLE]]` | `[[PENDIENTE]]` |
| Diseñar y entrenar IA embebida | `[[PENDIENTE DETALLE]]` | `[[PENDIENTE]]` |
| Implementar interfaz de usuario/representación | Parcial | Salida serie de clase detectada |
| Evaluar desempeño de la IA | `[[PENDIENTE DETALLE]]` | `[[PENDIENTE]]` |

---

## 7. Introducción

La digitalización de instrumentos musicales ha estado tradicionalmente ligada a captadores electromagnéticos, sistemas de análisis de audio o dispositivos externos de procesado. En paralelo, la evolución de los sensores MEMS y del cómputo embebido ha abierto una alternativa interesante: inferir información musical a partir de vibraciones y movimiento, reduciendo la dependencia de cadenas de audio convencionales.

El bajo eléctrico, por su función rítmica y armónica, ofrece un escenario atractivo para este tipo de aproximación. La posibilidad de identificar automáticamente la nota ejecutada puede habilitar aplicaciones de apoyo al aprendizaje, afinación asistida, control de efectos o interfaces MIDI no convencionales. Sin embargo, trasladar esta idea a un sistema práctico exige resolver varios retos: adquisición robusta, ruido, variabilidad de ejecución y restricciones de memoria/cómputo en el dispositivo objetivo.

Este proyecto aborda el problema desde una perspectiva aplicada. En lugar de centrarse únicamente en el entrenamiento de un modelo, se desarrolla una cadena completa que incluye decisiones de montaje físico del sensor, diseño del flujo de datos, clasificación y visualización de salida. El interés académico y profesional reside precisamente en esa integración de disciplinas.

Además, el proyecto tiene una dimensión iterativa: los resultados experimentales han condicionado decisiones clave del diseño. La comparación entre la colocación inicial del sensor en el mástil y configuraciones posteriores ilustra que el rendimiento final no depende solo del algoritmo, sino del sistema completo (instrumento, entorno, montaje y procesamiento).

Desde una perspectiva social y tecnológica, iniciativas como aiBass se enmarcan en una tendencia de democratización de herramientas inteligentes para creación musical. Un sistema de este tipo puede evolucionar hacia soluciones de asistencia en práctica instrumental, accesibilidad o interacción hombre-máquina en escenarios de bajo coste.

`[[FIGURA 1: Contexto del proyecto y flujo general de uso]]`

`[[AÑADIR 1-2 PÁRRAFOS PERSONALES SOBRE MOTIVACIÓN DEL ALUMNO]]`

---

## 8. Estado del arte

> **Nota de redacción:** en este apartado conviene citar fuentes concretas (papers, productos, documentación técnica).

### 8.1 Sistemas de detección de nota basados en audio

Los enfoques clásicos de detección de nota en instrumentos de cuerda suelen apoyarse en señal de audio. Estos sistemas aprovechan técnicas como análisis espectral, autocorrelación o modelos de aprendizaje sobre características acústicas. Su principal ventaja es la cercanía con la magnitud física directamente perceptible (frecuencia/pitch). Como limitación, pueden verse afectados por ruido ambiental, latencia de procesamiento y dependencia de una cadena de captación adecuada.

**Pros**:
- Madurez tecnológica y amplia bibliografía.
- Alta interpretabilidad en términos de frecuencia fundamental.

**Contras**:
- Sensibles al entorno acústico.
- Integración embebida condicionada por coste computacional y de captura.

`[[REFERENCIA 1]]`  
`[[REFERENCIA 2]]`

### 8.2 Afinadores y soluciones comerciales de ayuda al instrumentista

Existen múltiples dispositivos orientados a afinación o asistencia musical. Aunque no todos realizan clasificación de nota con el mismo objetivo que aiBass, su análisis permite comparar precisión, experiencia de usuario y viabilidad de producto.

`[[PRODUCTO A: nombre + descripción + precio + pros/contras]]`  
`[[PRODUCTO B: nombre + descripción + precio + pros/contras]]`  
`[[PRODUCTO C: nombre + descripción + precio + pros/contras]]`

### 8.3 Interfaces MIDI para guitarra/bajo

Los captadores y convertidores MIDI constituyen una familia relevante de referencia. Suelen buscar traducción de la interpretación a eventos musicales discretos, aunque la tecnología de captura puede diferir (hexafónica, audio digital, etc.).

Comparativamente, aiBass explora una vía alternativa basada en IMU, con potencial en simplicidad mecánica de integración y coste, a cambio de retos adicionales en robustez de clasificación.

`[[REFERENCIAS Y PRODUCTOS ESPECÍFICOS]]`

### 8.4 Sistemas basados en sensores inerciales (IMU) aplicados a música

Como referencia cercana en el dominio musical, puede citarse el TFG de **Madero Ayora (2024)** sobre un afinador electrónico embebido. Aunque comparte el objetivo de apoyo al instrumentista, su enfoque técnico se basa en una cadena diferente a aiBass, por lo que sirve como antecedente de contexto más que como réplica metodológica directa.

Entre los trabajos centrados explícitamente en IMU aplicadas a interpretación musical, **Freire et al. (2020)** evalúan la captura de gestos de rasgueo de guitarra con sensores inerciales frente a captura óptica de movimiento. Este trabajo aporta una validación experimental sólida de la utilidad de la IMU en tareas musicales reales, aunque su foco principal es la caracterización del gesto y no la clasificación de notas discretas.

En la línea de aprendizaje automático sobre datos de movimiento musical, **Dalmazzo y Ramírez (2019)** proponen la clasificación de gestos de arco en violín. El valor de este antecedente para aiBass está en demostrar que la señal gestual wearable puede transformarse en categorías interpretativas útiles mediante modelos de clasificación, si bien el problema objetivo difiere del reconocimiento de notas fundamentales en bajo.

De forma complementaria, **Provenzale et al. (2021)** estudian la técnica de arco en violinistas principiantes combinando MIMU y sensores de proximidad. Su contribución principal es metodológica: evidencian que la sensórica de bajo coste puede usarse para evaluación técnica y feedback, lo que refuerza la viabilidad de sistemas de asistencia musical basados en movimiento.

En síntesis, el estado del arte revisado confirma que las IMU son una tecnología válida para análisis musical y entrenamiento instrumental, pero deja menos cubierto el caso específico de **clasificación embebida de notas de bajo** en tiempo real. Precisamente ahí se sitúa la aportación de aiBass.

### 8.5 Aportación diferencial de aiBass

Frente al estado del arte revisado, aiBass aporta:

1. Integración de clasificación de notas fundamentales usando una cadena embebida.
2. Validación práctica del impacto del posicionamiento del sensor.
3. Prototipo funcional con salida en tiempo real por puerto serie.
4. Base para evolución hacia aplicaciones de afinación y/o interfaz tipo MIDI.

`[[AÑADIR COMPARATIVA EN TABLA: solución vs aiBass]]`

---

## 9. Tecnologías empleadas

### 9.1 Plataforma embebida principal

La plataforma principal de desarrollo es el **Discovery kit B-L4S5I-IOT01A**, basado en el microcontrolador **STM32L4S5VIT6** (familia STM32L4+, núcleo Arm Cortex-M4). Esta placa integra recursos suficientes para un prototipo de IA embebida y, al mismo tiempo, mantiene un enfoque de bajo consumo.

Desde el punto de vista práctico, su elección encaja muy bien con el alcance del TFG por tres motivos: integra sensórica directamente en placa (incluida la IMU LSM6DSL), dispone de herramientas de depuración y programación integradas (ST-LINK) y permite iterar rápido en firmware sin depender de hardware externo adicional.

Características destacables para este trabajo:

1. Microcontrolador STM32L4S5VIT6 con **2 MB de Flash** y **640 KB de RAM**.
2. Sensores integrados en la placa, incluyendo IMU, útiles para pruebas iniciales y validación rápida.
3. Conectividad y expansión (USB, cabeceras, conectores de expansión), que facilitan pruebas y evolución del prototipo.
4. Ecosistema software maduro (STM32Cube y documentación oficial), adecuado para desarrollo académico.

![Figura 2. Discovery kit B-L4S5I-IOT01A (fuente: STMicroelectronics)](https://www.st.com/bin/ecommerce/api/image.PF270120.en.feature-description-include-personalized-no-cpn-large.jpg)

### 9.2 Sensor inercial

La IMU empleada es la **LSM6DSL**, un módulo inercial de 6 ejes (acelerómetro 3D + giroscopio 3D). Para este proyecto resulta especialmente adecuada por su equilibrio entre prestaciones, consumo y disponibilidad dentro del propio kit de desarrollo.

De acuerdo con la documentación del fabricante, el sensor permite:

1. Rangos de aceleración de **±2/±4/±8/±16 g**.
2. Rangos de velocidad angular de **±125/±245/±500/±1000/±2000 dps**.
3. Operación en modo de altas prestaciones con consumo reducido (orden de mA bajo).
4. Funciones orientadas a adquisición continua y almacenamiento por lotes para análisis temporal.

En aiBass, la LSM6DSL actúa como fuente principal de señal para construir ventanas temporales que posteriormente se clasifican en las clases musicales objetivo.

### 9.3 Firmware y herramientas de desarrollo

El desarrollo firmware se ha apoyado en el ecosistema STM32 para configurar periféricos, adquirir señal inercial y publicar resultados por serie. En términos funcionales, el firmware implementa tres bloques: captura de muestras IMU, preparación de entrada para inferencia y envío de etiqueta de clase detectada.

Herramientas y componentes de trabajo empleados:

1. Entorno STM32 para configuración y compilación del proyecto embebido.
2. Drivers/periféricos para lectura del sensor inercial y comunicación UART.
3. Flujo de depuración iterativo con salida serie para validación del comportamiento en tiempo real.

Además, el proyecto incorporó una fase específica de evaluación de una alternativa tecnológica basada en **Rust + Embassy** (aprox. 100 h), inicialmente descartada para el cierre del prototipo, pero considerada relevante para evolución futura.

### 9.4 Pipeline de IA

La cadena de IA del proyecto sigue un enfoque clásico de clasificación supervisada adaptado a restricciones embebidas:

1. Captura de señal inercial y segmentación en ventanas temporales.
2. Etiquetado por clases objetivo (E, A, D, G y ruido/silencio).
3. Entrenamiento iterativo fuera del microcontrolador.
4. Integración del modelo resultante en el firmware para inferencia local.

Como referencia de herramientas para edge AI en entorno STM32, se ha tenido en cuenta **NanoEdge AI Studio**, alineado con la línea de IA embebida del fabricante.

### 9.5 Interfaz de salida

La interfaz actual de interacción es **serial (UART)**, orientada a depuración y validación rápida de resultados. La salida se emite en formato textual con la clase detectada en cada instante de decisión, lo que permite revisar de forma inmediata la estabilidad del sistema durante las pruebas.

Este enfoque simplifica el ciclo de experimentación: facilita observar errores de clasificación, comparar comportamientos entre montajes físicos y ajustar el pipeline sin necesidad de una interfaz gráfica compleja.

### 9.6 Tecnologías auxiliares

Como soporte al desarrollo se han utilizado tecnologías auxiliares de uso habitual en proyectos software/embebidos:

1. **Git y GitHub** para control de versiones y trazabilidad del desarrollo.
2. Herramientas de tratamiento de datos para organizar muestras y resultados experimentales.
3. Documentación técnica del ecosistema STM32 y de la sensórica empleada para guiar decisiones de diseño.

---

## 10. Arquitectura del sistema

### 10.1 Visión global

La arquitectura funcional de aiBass puede representarse como una tubería de procesamiento:

1. Captura de datos inerciales.
2. Ventaneado y preprocesado.
3. Inferencia del modelo de clasificación.
4. Postprocesado de la decisión.
5. Publicación del resultado por serie.

`[[FIGURA 4: Diagrama de bloques a página completa]]`

### 10.2 Bloques principales

#### Bloque A — Captura IMU
Adquiere muestras del LSM6DSL con la configuración de muestreo definida.

#### Bloque B — Preprocesado
Normaliza/estructura los datos para formar la entrada de inferencia.

#### Bloque C — Clasificación
Genera la etiqueta de clase entre {E, A, D, G, ruido}.

#### Bloque D — Salida y monitorización
Emite la clase detectada por puerto serie y facilita la depuración.

### 10.3 Relación tecnología-bloque

| Bloque | Tecnología principal | Observaciones |
|---|---|---|
| A | LSM6DSL + STM32L4S5VI | Adquisición de señal |
| B | Firmware `[[PENDIENTE]]` | Preparación de entrada |
| C | Modelo IA `[[PENDIENTE]]` | Clasificación |
| D | UART/Serial | Diagnóstico y salida |

---

## 11. Desarrollo

> Este capítulo puede ser el más extenso. Aquí está preparado para que lo lleves a 15-20 páginas.

### 11.1 Enfoque de desarrollo iterativo

El desarrollo se abordó de manera incremental: primero asegurar captura estable de señal, después construir un primer clasificador funcional y, finalmente, cerrar el bucle de inferencia en tiempo real con salida observable.

### 11.2 Fase de investigación en Rust + Embassy (aprox. 100 horas)

Antes de consolidar la implementación actual, se dedicó una fase extensa a investigar la viabilidad de desarrollar el firmware con **Rust** sobre el ecosistema **Embassy**. Esta fase ocupó cerca de **100 horas** dentro de una dedicación total aproximada de **300 horas** del proyecto.

Los objetivos de esta línea eran:

1. Evaluar madurez de herramientas y flujo de desarrollo.
2. Analizar la integración con periféricos necesarios para captura y salida.
3. Estimar el impacto en tiempos de desarrollo frente al calendario del TFG.

Tras la evaluación, se decidió **abandonar** esta alternativa para priorizar la entrega funcional del sistema en el alcance temporal disponible.

`[[AÑADIR DETALLE: principales bloqueos encontrados (toolchain, drivers, curva de aprendizaje, integración IA, etc.)]]`

### 11.3 Primera etapa: montaje inicial y validación temprana

En una fase inicial se evaluó la colocación de la IMU sobre el mástil del bajo. Esta decisión partía de la hipótesis de cercanía a la fuente de vibración de la cuerda. No obstante, los resultados observados mostraron una fiabilidad insuficiente para la detección robusta de clases objetivo.

`[[AÑADIR DETALLE: qué se observó exactamente y cómo se midió la fiabilidad]]`

### 11.4 Cambio de estrategia de montaje

Tras los resultados anteriores, se modificó el montaje para probar la IMU en una configuración alternativa (sobre el amplificador). Esta decisión responde a una lógica de ingeniería experimental: cuando la señal útil no es estable en una configuración, se replantea el punto de medida para mejorar separación entre clases.

`[[FIGURA 5: Comparativa de montajes (mástil vs amplificador)]]`

`[[AÑADIR: ventajas e inconvenientes detectados en la nueva configuración]]`

### 11.5 Construcción del dataset

`[[PENDIENTE DE DATOS REALES]]`

Estructura sugerida para este subapartado:

1. Protocolo de adquisición por clase.
2. Número de muestras por clase.
3. Condiciones de captura (instrumento, dinámica, entorno).
4. Formato de almacenamiento.
5. Etiquetado y control de calidad.

`[[TABLA 1: Distribución de muestras por clase]]`

### 11.6 Preprocesado y generación de características

`[[DESCRIBIR: filtrado, normalización, segmentación temporal, etc., solo lo que realmente uses]]`

`[[FIGURA 6: Flujo de preprocesado]]`

### 11.7 Entrenamiento y validación del modelo

`[[PENDIENTE DE DATOS REALES]]`

Contenido a completar:

- Tipo de modelo y arquitectura.
- División entrenamiento/validación/prueba.
- Hiperparámetros relevantes.
- Métricas empleadas.
- Criterio de selección del modelo final.

`[[TABLA 2: Resultados por versión de modelo]]`

### 11.8 Integración embebida

En esta fase se integra el modelo en la plataforma STM32L4S5VI y se enlaza con el pipeline de captura y salida serie.

`[[AÑADIR: detalles de memoria, latencia y optimizaciones si aplican]]`

### 11.9 Lógica de clasificación y salida serie

El sistema actual genera etiquetas de clase para:

- **E**
- **A**
- **D**
- **G**
- **ruido/silencio**

El resultado se muestra por puerto serie para validación y seguimiento de comportamiento en tiempo real.

`[[FIGURA 7: Captura de terminal serie mostrando detecciones]]`

### 11.10 Problemas encontrados y soluciones aplicadas

`[[DESCRIBIR PROBLEMAS REALES EN FORMATO: problema -> impacto -> solución]]`

Plantilla:

| Problema | Impacto | Solución aplicada | Resultado |
|---|---|---|---|
| `[[P1]]` | `[[...]]` | `[[...]]` | `[[...]]` |
| `[[P2]]` | `[[...]]` | `[[...]]` | `[[...]]` |

### 11.11 Alternativas desechadas

`[[AÑADIR DECISIONES TÉCNICAS DESCARTADAS Y MOTIVOS]]`

Ejemplos de justificación esperada:

- Complejidad excesiva para el alcance del TFG.
- Coste computacional incompatible con la plataforma.
- Falta de mejora clara frente a la solución adoptada.
- **Rust + Embassy (descartado tras ~100 h)**: inversión alta para el tiempo disponible y menor contribución inmediata al objetivo funcional del prototipo.

---

## 12. Pruebas del sistema

### 12.1 Objetivo del plan de pruebas

Verificar el comportamiento del sistema en términos de:

1. Detección de clases objetivo (E, A, D, G, ruido).
2. Estabilidad temporal de la salida.
3. Robustez ante variaciones de ejecución.

### 12.2 Diseño de pruebas funcionales

| ID | Escenario | Entrada esperada | Salida esperada | Resultado |
|---|---|---|---|---|
| PF-01 | Cuerda asociada a E | Señal clase E | Etiqueta E | `[[PENDIENTE]]` |
| PF-02 | Cuerda asociada a A | Señal clase A | Etiqueta A | `[[PENDIENTE]]` |
| PF-03 | Cuerda asociada a D | Señal clase D | Etiqueta D | `[[PENDIENTE]]` |
| PF-04 | Cuerda asociada a G | Señal clase G | Etiqueta G | `[[PENDIENTE]]` |
| PF-05 | Sin nota válida | Ruido/silencio | Etiqueta ruido | `[[PENDIENTE]]` |

### 12.3 Pruebas de robustez

`[[AÑADIR: variación de intensidad, repetibilidad, sesiones distintas, etc.]]`

### 12.4 Métricas e indicadores

`[[AÑADIR: exactitud, matriz de confusión, latencia, tasa de falsos positivos, etc., solo si los has medido]]`

`[[FIGURA 8: Matriz de confusión]]`  
`[[FIGURA 9: Curvas o histogramas relevantes]]`

### 12.5 Discusión de resultados

`[[INTERPRETAR RESULTADOS REALES: qué funciona bien, qué falla y por qué]]`

---

## 13. Planificación temporal

### 13.1 Fases del proyecto

Dedicación total estimada del proyecto (desarrollo): **~300 horas**.

| Hito | Descripción | Horas (h) | % sobre total | Rango acumulado (h) |
|---|---|---:|---:|---|
| H1 | Investigación de viabilidad con Rust + Embassy (descartada) | **100** | **33.3%** | 0-100 |
| H2 | Adquisición y organización de datos | 58 | 19.3% | 100-158 |
| H3 | Entrenamiento y ajuste del modelo IA | 52 | 17.3% | 158-210 |
| H4 | Integración embebida + salida serie | 42 | 14.0% | 210-252 |
| H5 | Pruebas, depuración y ajustes finales | 28 | 9.3% | 252-280 |
| H6 | Redacción técnica de memoria y documentación | 20 | 6.7% | 280-300 |
| **TOTAL** |  | **300** | **100%** | 0-300 |

### 13.2 Representación gráfica

**Diagrama de Gantt (escala en horas acumuladas, sin fechas):**

```text
Escala horizontal: 0h ───────────────────────────────────────────────────── 300h

H1  Investigación Rust+Embassy   [0 ──────────────────────── 100]
H2  Adquisición de datos         [100 ─────────── 158]
H3  Entrenamiento IA             [158 ────────── 210]
H4  Integración embebida         [210 ──────── 252]
H5  Pruebas y ajuste             [252 ───── 280]
H6  Redacción memoria            [280 ─── 300]
```

`[[FIGURA 10: Si se quiere formato visual, convertir este Gantt textual a imagen en Google Docs con barras horizontales]]`

### 13.3 Análisis temporal

La fase de mayor peso temporal fue **H1 (100 h, 33.3%)**, dedicada a investigar la viabilidad de una implementación en Rust con Embassy. Aunque esta línea no se mantuvo en la solución final, concentró una parte relevante del esfuerzo por curva de aprendizaje, evaluación de herramientas y pruebas de integración.

El segundo bloque en tiempo fue la preparación de datos (**H2, 58 h**), seguido del entrenamiento del modelo (**H3, 52 h**). Esto es coherente con un proyecto de IA embebida, donde la calidad del dato y la iteración del modelo condicionan fuertemente el resultado final.

La integración en firmware y salida serie (**H4, 42 h**) supuso menos carga que las fases exploratorias iniciales, al apoyarse en una arquitectura ya definida. Las últimas fases (**H5-H6**) consumieron menos horas al centrarse en consolidación, validación y redacción, con menor incertidumbre técnica que en etapas anteriores.

---

## 14. Costes

### 14.1 Coste de materiales (prototipo)

| Concepto | Unidades | Coste unitario | Subtotal |
|---|---:|---:|---:|
| Discovery kit **B-L4S5I-IOT01A** (incluye IMU LSM6DSL) | 1 | 50.00 € | 50.00 € |
| Cable USB / alimentación / conexión | 1 | 6.00 € | 6.00 € |
| Material auxiliar de montaje (cinta, fijación, consumibles) | 1 | 4.00 € | 4.00 € |
| **TOTAL MATERIAL** |  |  | **60.00 €** |

### 14.2 Coste de personal

Para estimar el coste de personal se utiliza una métrica equivalente a **COCOMO simple** basada en personas-mes.

Suposiciones de cálculo:

1. Proyecto desarrollado por **1 persona**.
2. Dedicación total técnica: **300 horas**.
3. Jornada mensual de referencia: **150 h/mes**.
4. Coste mensual imputado del perfil técnico: **2,100 €/mes**.

| Concepto | Valor |
|---|---:|
| Horas de desarrollo (totales) | **300 h** |
| Horas dedicadas a investigación Rust + Embassy (descartada) | **100 h** |
| Personas-mes estimadas (300 / 150) | **2.00 PM** |
| Coste mensual por persona | **2,100 €** |
| **TOTAL PERSONAL (2.00 × 2,100)** | **4,200.00 €** |

### 14.3 Coste total de prototipo

**Coste prototipo = Coste material + Coste personal = 60.00 € + 4,200.00 € = 4,260.00 €**

### 14.4 Estimación de producción en serie

Estimación para una tirada de **1,500 unidades**, aplicando un descuento de material del **18%** y un margen comercial del **25%**.

| Concepto | Valor |
|---|---:|
| Unidades | **1,500** |
| Coste base de material por unidad (referencia kit) | **50.00 €** |
| Descuento material por volumen | **18%** |
| Material estimado por unidad tras descuento | **41.00 €** |
| Repercusión de coste de desarrollo por unidad (4,200 / 1,500) | **2.80 €** |
| Coste estimado por unidad antes de margen | **43.80 €** |
| Margen de beneficio aplicado | **25%** |
| **Precio final estimado de venta al público por unidad** | **54.75 €** |

---

## 15. Conclusiones

El proyecto aiBass ha permitido construir una base funcional para detección de notas en bajo mediante sensórica inercial e inferencia embebida. En su estado actual, se ha logrado la clasificación de cinco clases de interés (E, A, D, G y ruido/silencio), junto con la visualización de resultados en serie para validación operativa.

Desde el punto de vista de ingeniería, uno de los aprendizajes clave ha sido la importancia del enfoque sistémico: el rendimiento no depende únicamente del modelo, sino también del montaje físico, la calidad de la adquisición y la consistencia del pipeline de procesamiento.

`[[AÑADIR CIERRE PERSONAL DEL ALUMNO: aprendizaje, dificultades, valoración global]]`

---

## 16. Trabajo futuro

Como evolución natural de aiBass, se plantean varias líneas de continuidad que permitirían pasar de un prototipo funcional a un sistema más sólido y cercano a uso real.

### 16.1 Ampliación de clases y resolución musical

El sistema actual distingue E, A, D, G y ruido/silencio. Una extensión directa es aumentar la resolución de clasificación para incluir más notas, diferentes posiciones en el diapasón y, potencialmente, técnicas de ejecución (ataque, palm mute, etc.). Esta ampliación exigiría un dataset más amplio y equilibrado, además de una revisión del modelo para mantener robustez.

### 16.2 Mejora de robustez y generalización

Se propone reforzar la robustez frente a variaciones de interpretación, dinámica de ataque, instrumento y entorno físico. Para ello, sería útil diseñar campañas de captura en distintas condiciones, introducir validación cruzada más exigente y aplicar técnicas de regularización/normalización orientadas a mejorar generalización fuera del entorno de entrenamiento.

### 16.3 Optimización del montaje físico del sensor

La evolución del proyecto ha mostrado que el punto de montaje impacta fuertemente en la calidad de señal. Como trabajo futuro, se plantea un estudio sistemático de ubicaciones con criterios cuantitativos de discriminación entre clases, junto con una solución mecánica estable de fijación para reducir variaciones no controladas entre sesiones.

### 16.4 Evolución de la interfaz de usuario

La salida serie actual es suficiente para depuración, pero limitada para uso final. Una línea de mejora sería desarrollar una interfaz de mayor nivel (aplicación de escritorio o móvil) que muestre historial de detecciones, métricas de confianza y herramientas de apoyo al aprendizaje o afinación.

### 16.5 Integración musical avanzada (MIDI/DAW)

Una evolución especialmente interesante es mapear la clasificación a eventos MIDI para controlar sintetizadores o integrarse con estaciones de audio digital (DAW). Esta línea abriría aplicaciones directas en producción musical y permitiría evaluar el sistema en contextos de interpretación en tiempo real.

### 16.6 Revisión tecnológica de la línea Rust + Embassy

Aunque la investigación inicial en Rust + Embassy se descartó en esta etapa por coste temporal, actualmente ya se dispone de una base técnica más sólida en ese ecosistema. Por ello, se plantea como línea futura prioritaria una reimplementación progresiva del firmware en Rust/Embassy, al menos en módulos críticos de adquisición y comunicaciones.

Esta propuesta está respaldada por experiencia reciente en otro proyecto del autor, donde se realizó una comparación directa **C/FreeRTOS vs Rust/Embassy** sobre una aplicación equivalente de UART. En esa comparación se observaron mejoras relevantes en huella de memoria y tamaño binario para la implementación en Rust, lo que refuerza su interés para futuras iteraciones de aiBass (véase referencia bibliográfica específica al documento [*COMPARISON.md*](https://github.com/AlfEspadero/SETR2-practica4-rust/blob/main/COMPARISON.md)).

### 16.7 Validación con usuarios

Finalmente, se propone realizar pruebas con bajistas de distintos niveles para medir utilidad percibida, facilidad de uso y estabilidad en escenarios reales de práctica. Esta validación serviría para priorizar mejoras con impacto directo en la experiencia de usuario.

---

## 17. Bibliografía

Espadero García, A. (s. f.). *aiBass* [Repositorio de software]. GitHub. Recuperado el 9 de mayo de 2026, de https://github.com/AlfEspadero/aiBass

Espadero García, A. (s. f.). *STM32 Embedded Development: C/FreeRTOS vs Rust/Embassy* [Archivo Markdown]. GitHub. Recuperado el 9 de mayo de 2026, de https://github.com/AlfEspadero/SETR2-practica4-rust/blob/main/COMPARISON.md

Madero Ayora, M. J. (2024). *Desarrollo de un afinador electrónico para instrumento musical basado en DSP TMS320LF28335* [Trabajo Fin de Grado, Universidad de Sevilla]. idUS. https://idus.us.es/items/bf855f16-ea4a-4bed-b7f7-4d550b9813d5

Madero Ayora, M. J. (2024). *Desarrollo de un afinador electrónico para instrumento musical basado en DSP TMS320LF28335* [Documento PDF]. Universidad de Sevilla. https://idus.us.es/server/api/core/bitstreams/678a1ea6-3083-4eab-a3ee-350e4bd36843/content

Freire, S., Santos, G., Armondes, A., Meneses, E. A. L., & Wanderley, M. M. (2020). *Evaluation of inertial sensor data by a comparison with optical motion capture data of guitar strumming gestures*. *Sensors, 20*(19), Article 5722. https://doi.org/10.3390/s20195722

Dalmazzo, D., & Ramírez, R. (2019). *Bowing gestures classification in violin performance: A machine learning approach*. *Frontiers in Psychology, 10*, Article 344. https://doi.org/10.3389/fpsyg.2019.00344

Provenzale, C., Di Stefano, N., Noccaro, A., & Taffoni, F. (2021). *Assessing the bowing technique in violin beginners using MIMU and optical proximity sensors: A feasibility study*. *Sensors, 21*(17), Article 5817. https://doi.org/10.3390/s21175817

STMicroelectronics. (s. f.). *B-L4S5I-IOT01A Discovery kit for IoT node*. Recuperado el 9 de mayo de 2026, de https://www.st.com/en/evaluation-tools/b-l4s5i-iot01a.html

STMicroelectronics. (s. f.). *Board photo for B-L4S5I-IOT01A* [Imagen]. Recuperado el 9 de mayo de 2026, de https://www.st.com/bin/ecommerce/api/image.PF270120.en.feature-description-include-personalized-no-cpn-large.jpg

STMicroelectronics. (s. f.). *LSM6DSL: iNEMO inertial module*. Recuperado el 9 de mayo de 2026, de https://www.st.com/en/mems-and-sensors/lsm6dsl.html

STMicroelectronics. (s. f.). *NanoEdge AI Studio*. Recuperado el 9 de mayo de 2026, de https://stm32ai.st.com/nanoedge-ai/

STMicroelectronics. (s. f.). *STM32Cube embedded software*. Recuperado el 9 de mayo de 2026, de https://www.st.com/en/embedded-software/stm32cube-embedded-software.html

STMicroelectronics. (s. f.). *STM32L4+ Series*. Recuperado el 9 de mayo de 2026, de https://www.st.com/en/microcontrollers-microprocessors/stm32l4-plus-series.html

STMicroelectronics. (s. f.). *AI:NanoEdge AI Studio* [Wiki técnica]. Recuperado el 9 de mayo de 2026, de https://wiki.st.com/stm32mcu/wiki/AI:NanoEdge_AI_Studio

---

## 18. Anexos

### Anexo A — Glosario

`[[DEFINIR TÉRMINOS: IMU, inferencia, ventana temporal, clase, etc.]]`

### Anexo B — Manual de usuario (versión prototipo)

`[[PASOS PARA PONER EN MARCHA EL SISTEMA Y LEER LA SALIDA SERIE]]`

### Anexo C — Manual de instalación/compilación

`[[TOOLCHAIN, CONFIGURACIÓN, CARGA EN PLACA]]`

### Anexo D — Evidencias adicionales

`[[CAPTURAS EXTRA, LOGS, TABLAS EXTENDIDAS]]`

---

## Checklist de sustituciones antes de entrega

- `[[PENDIENTE]]` rellenados.
- Figuras insertadas y numeradas.
- Tablas completadas con datos reales.
- Referencias bibliográficas completas.
- Índice e índice de figuras regenerados.
- Revisión ortográfica y de estilo final.
