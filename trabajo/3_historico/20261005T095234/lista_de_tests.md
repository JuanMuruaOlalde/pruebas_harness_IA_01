# Lista de tests: interfaz gráfica de usuario para manejar el simulador

Los requisitos, los supuestos (U1–U17), los textos exactos de los mensajes y los ejemplos de referencia están en `trabajo/2_en_curso/lista_de_requisitos_y_casos_de_uso.md`. Conviene leer sobre todo los apartados 4 (R1–R5) y 6 (ejemplos de referencia).

Salvo que el nombre del test diga otra cosa, los tests del presentador usan:
- el edificio estándar: plantas de la -2 a la 7, 3 ascensores, 2 s por planta, y 4 s de arranque y de parada;
- la configuración estándar del control de tráfico: 28 días de histórico y 30 s de reposo;
- el histórico en memoria;
- como fecha y hora de inicio, el lunes 5 de octubre de 2026 a las 08:00:00.

Como `pasar_el_tiempo` avanza como máximo 1 s por llamada, conviene una función auxiliar que avance `n` segundos llamando `n` veces a `pasar_el_tiempo(1 s)`.

La capa de dibujo (`vista_con_egui.rs`) y `main.rs` no llevan tests automáticos (U14). Se comprueban a mano con la lista del apartado 8 de los requisitos.

No se modifica ninguno de los 123 tests existentes. Los tests nuevos se pueden añadir a los módulos de tests que ya existen.

Ubicación: un módulo `#[cfg(test)] mod tests` en el archivo de cada tipo. Los encabezados indican el archivo.

## Mensajes legibles de los errores del dominio (`src/dominio/errores.rs`)

- [x] fn los_errores_de_configuracion_se_muestran_con_un_mensaje_legible_en_espanol()
- [x] fn los_errores_de_simulacion_se_muestran_con_un_mensaje_legible_en_espanol()

## Mensajes legibles de los errores del histórico (`src/dominio/historico_de_movimientos.rs`)

- [x] fn los_errores_del_historico_de_movimientos_se_muestran_con_un_mensaje_legible_que_incluye_sus_datos()

## Mensajes legibles de los errores del control de tráfico (`src/aplicacion/control_de_trafico.rs`)

- [x] fn los_errores_del_control_de_trafico_se_muestran_con_el_mensaje_del_error_que_contienen()
- [x] fn los_errores_del_dominio_y_del_control_de_trafico_implementan_std_error_error()

## Componentes de la fecha y hora (`src/dominio/fecha_y_hora.rs`)

- [x] fn los_componentes_de_una_fecha_y_hora_son_los_indicados_al_crearla()
- [x] fn el_segundo_de_una_fecha_y_hora_no_incluye_la_fraccion_de_segundo()

## Presentador: estado inicial y estructura del modelo de vista (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn al_crear_el_presentador_no_hay_mensaje_para_el_usuario()
- [x] fn el_modelo_de_vista_tiene_una_planta_por_cada_planta_del_edificio_de_la_mas_alta_a_la_mas_baja()
- [x] fn cada_planta_del_modelo_de_vista_tiene_una_celda_de_hueco_por_ascensor()
- [x] fn el_modelo_de_vista_tiene_un_ascensor_por_cada_ascensor_del_edificio_ordenados_por_identificador()
- [x] fn la_botonera_de_cada_ascensor_tiene_un_boton_por_planta_de_la_mas_alta_a_la_mas_baja()
- [x] fn al_iniciar_todos_los_ascensores_aparecen_parados_en_la_planta_principal()

## Presentador: posición y marcha de los ascensores (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn la_celda_del_hueco_de_la_planta_actual_de_un_ascensor_muestra_el_ascensor_con_su_marcha()
- [x] fn la_celda_del_hueco_de_la_planta_de_destino_muestra_el_destino_mientras_el_ascensor_no_ha_llegado()
- [x] fn las_celdas_del_hueco_de_las_demas_plantas_estan_vacias()
- [x] fn un_ascensor_que_va_hacia_una_planta_mas_alta_esta_subiendo()
- [x] fn un_ascensor_que_va_hacia_una_planta_mas_baja_esta_bajando()
- [x] fn un_ascensor_en_la_parada_de_su_planta_de_destino_esta_llegando()
- [x] fn un_ascensor_que_ha_terminado_su_desplazamiento_queda_parado_en_la_planta_de_destino()
- [x] fn la_descripcion_de_un_ascensor_parado_indica_su_planta()
- [x] fn la_descripcion_de_un_ascensor_en_marcha_indica_su_planta_actual_su_sentido_y_su_destino()

## Presentador: botones de llamada y botoneras (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn el_boton_de_llamada_de_una_planta_sin_llamada_en_curso_esta_apagado()
- [x] fn el_boton_de_llamada_de_una_planta_con_llamada_pendiente_esta_encendido()
- [x] fn el_boton_de_llamada_de_una_planta_hacia_la_que_se_desplaza_un_ascensor_esta_encendido()
- [x] fn el_boton_de_llamada_se_apaga_cuando_el_ascensor_queda_parado_en_la_planta()
- [x] fn los_botones_de_la_botonera_de_un_ascensor_parado_estan_habilitados_y_apagados()
- [x] fn los_botones_de_la_botonera_de_un_ascensor_que_se_desplaza_estan_deshabilitados()
- [x] fn el_boton_de_la_planta_de_destino_de_la_botonera_esta_encendido_mientras_el_ascensor_se_desplaza()

## Presentador: textos de la fecha y hora y de las llamadas pendientes (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn la_fecha_y_hora_se_muestra_con_el_dia_de_la_semana_y_sin_fraccion_de_segundo()
- [x] fn los_dias_de_la_semana_se_muestran_con_su_nombre_en_espanol()
- [x] fn sin_llamadas_pendientes_el_texto_de_las_llamadas_pendientes_lo_indica()
- [x] fn las_llamadas_pendientes_se_muestran_por_orden_de_llegada()

## Presentador: botón de llamada (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn pulsar_un_boton_de_llamada_envia_el_ascensor_libre_mas_cercano_a_esa_planta()
- [x] fn el_mensaje_de_una_llamada_atendida_indica_la_planta_y_el_ascensor_que_la_atiende()
- [x] fn el_mensaje_de_una_llamada_sin_ascensores_libres_indica_que_queda_pendiente()
- [x] fn el_mensaje_de_una_llamada_repetida_indica_que_ya_estaba_en_curso()
- [x] fn el_mensaje_de_una_llamada_a_una_planta_inexistente_es_un_error()
- [x] fn si_falla_el_historico_al_pulsar_un_boton_de_llamada_el_mensaje_es_un_error_aunque_el_ascensor_acuda()

## Presentador: botonera (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn pulsar_un_boton_de_la_botonera_envia_ese_ascensor_a_esa_planta()
- [x] fn el_mensaje_de_una_orden_de_la_botonera_aceptada_indica_el_ascensor_y_la_planta_de_destino()
- [x] fn el_mensaje_de_una_orden_de_la_botonera_a_la_planta_en_la_que_esta_parado_el_ascensor_indica_que_ya_esta_en_ella()
- [x] fn el_mensaje_de_una_orden_de_la_botonera_a_un_ascensor_que_se_desplaza_es_un_error_de_ascensor_ocupado()
- [x] fn una_accion_nueva_sustituye_el_mensaje_de_la_accion_anterior()

## Presentador: paso del tiempo (`src/adaptadores/interfaz_grafica/presentador.rs`)

- [x] fn pasar_el_tiempo_avanza_la_simulacion_el_tiempo_real_transcurrido()
- [x] fn pasar_el_tiempo_avanza_la_simulacion_como_maximo_un_segundo_aunque_haya_transcurrido_mas_tiempo_real()
- [x] fn pasar_el_tiempo_sin_errores_conserva_el_ultimo_mensaje_para_el_usuario()
- [x] fn si_falla_el_historico_al_pasar_el_tiempo_el_mensaje_para_el_usuario_es_un_error()
