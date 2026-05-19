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

La literatura de interacción musical con IMU suele enfocarse en control gestual, seguimiento de movimiento o mapeos expresivos. El uso específico para clasificación de nota en bajo es menos habitual y, por ello, aporta interés diferencial.

`[[RESUMEN DE 2-3 TRABAJOS ACADÉMICOS CERCANOS]]`

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

**Microcontrolador/placa**: **STM32L4S5VI**.

`[[DESCRIBIR: CPU, memoria, periféricos usados, consumo, por qué se eligió]]`

`[[FIGURA 2: Foto de la placa]]`

### 9.2 Sensor inercial

**IMU**: **LSM6DSL**.

`[[DESCRIBIR: acelerómetro/giroscopio, rango, frecuencia de muestreo usada, interfaz (I2C/SPI), resolución]]`

`[[FIGURA 3: Foto del sensor / ubicación de montaje]]`

### 9.3 Firmware y herramientas de desarrollo

- Lenguaje principal: `[[PENDIENTE]]`
- Entorno de desarrollo: `[[PENDIENTE]]`
- Librerías HAL/Drivers: `[[PENDIENTE]]`
- Sistema de adquisición/registro de muestras: `[[PENDIENTE]]`
- Línea investigada y descartada: **Rust + Embassy** (aprox. **100 h** de investigación).

### 9.4 Pipeline de IA

- Framework de entrenamiento: `[[PENDIENTE]]`
- Tipo de modelo: `[[PENDIENTE]]`
- Formato de exportación a embebido: `[[PENDIENTE]]`
- Estrategia de inferencia en dispositivo: `[[PENDIENTE]]`

### 9.5 Interfaz de salida

- Medio actual: **salida serie**.
- Formato de mensajes: `[[PENDIENTE]]`
- Frecuencia de actualización: `[[PENDIENTE]]`

### 9.6 Tecnologías auxiliares

`[[AÑADIR: scripts de procesado, herramientas de visualización, control de versiones, etc.]]`

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

`[[PENDIENTE FECHAS REALES]]`

Plantilla de hitos:

| Hito | Descripción | Inicio | Fin | Dedicación (h) |
|---|---|---|---|---|
| H1 | Investigación de viabilidad con Rust + Embassy (descartada) | `[[...]]` | `[[...]]` | **100** |
| H2 | Adquisición de datos | `[[...]]` | `[[...]]` | `[[...]]` |
| H3 | Entrenamiento IA | `[[...]]` | `[[...]]` | `[[...]]` |
| H4 | Integración embebida + salida serie | `[[...]]` | `[[...]]` | `[[...]]` |
| H5 | Pruebas y ajuste | `[[...]]` | `[[...]]` | `[[...]]` |
| H6 | Redacción memoria | `[[...]]` | `[[...]]` | `[[...]]` |
| **TOTAL** |  |  |  | **300** |

### 13.2 Representación gráfica

`[[FIGURA 10: Diagrama de Gantt]]`

### 13.3 Análisis temporal

`[[EXPLICAR: qué tareas consumieron más tiempo y por qué]]`

---

## 14. Costes

### 14.1 Coste de materiales (prototipo)

| Concepto | Unidades | Coste unitario | Subtotal |
|---|---:|---:|---:|
| `[[Placa STM32L4S5VI / kit]]` | `[[...]]` | `[[...]] €` | `[[...]] €` |
| `[[Sensor IMU LSM6DSL]]` | `[[...]]` | `[[...]] €` | `[[...]] €` |
| `[[Cableado/montaje]]` | `[[...]]` | `[[...]] €` | `[[...]] €` |
| `[[Otros]]` | `[[...]]` | `[[...]] €` | `[[...]] €` |
| **TOTAL MATERIAL** |  |  | `[[...]] €` |

### 14.2 Coste de personal

`[[INDICAR MODELO DE ESTIMACIÓN (COCOMO SIMPLE U OTRO) Y PARÁMETROS REALES]]`

Referencia conocida para cálculo: **300 horas** de dedicación total de desarrollo.

| Concepto | Valor |
|---|---:|
| Horas de desarrollo (totales) | **300 h** |
| Horas dedicadas a investigación Rust + Embassy (descartada) | **100 h** |
| Personas-mes estimadas | `[[...]]` |
| Coste mensual por persona | `[[...]] €` |
| **TOTAL PERSONAL** | `[[...]] €` |

### 14.3 Coste total de prototipo

**Coste prototipo = Coste material + Coste personal = `[[...]] €`**

### 14.4 Estimación de producción en serie

`[[EJEMPLO REQUERIDO POR PLANTILLA: 1000-2000 uds, descuento material 15-20%, margen 20-30%]]`

| Concepto | Valor |
|---|---:|
| Unidades | `[[...]]` |
| Descuento material | `[[...]] %` |
| Margen beneficio | `[[...]] %` |
| Precio final estimado por unidad | `[[...]] €` |

---

## 15. Conclusiones

El proyecto aiBass ha permitido construir una base funcional para detección de notas en bajo mediante sensórica inercial e inferencia embebida. En su estado actual, se ha logrado la clasificación de cinco clases de interés (E, A, D, G y ruido/silencio), junto con la visualización de resultados en serie para validación operativa.

Desde el punto de vista de ingeniería, uno de los aprendizajes clave ha sido la importancia del enfoque sistémico: el rendimiento no depende únicamente del modelo, sino también del montaje físico, la calidad de la adquisición y la consistencia del pipeline de procesamiento.

`[[AÑADIR CIERRE PERSONAL DEL ALUMNO: aprendizaje, dificultades, valoración global]]`

---

## 16. Trabajo futuro

Líneas propuestas para continuidad del proyecto:

1. Ampliar el número de clases (más notas/posiciones).
2. Mejorar robustez frente a variaciones de interpretación y entorno.
3. Refinar montaje y encapsulado del sensor para uso real en instrumento.
4. Incorporar una interfaz de usuario más avanzada (más allá de terminal serie).
5. Evolucionar hacia salida MIDI o integración con DAW.
6. Validar el sistema con más usuarios e instrumentos.

`[[PRIORIZAR SEGÚN TIEMPO/COSTE REAL DEL TFG]]`

---

## 17. Bibliografía

`[[AÑADIR REFERENCIAS REALES EN FORMATO CONSISTENTE (APA/IEEE/OTRO)]]`

Plantilla recomendada:

1. Autor(es), "Título", fuente/editorial, año.
2. Autor(es), "Título", revista/conferencia, año.
3. Nombre de recurso web, URL, fecha de consulta.

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
