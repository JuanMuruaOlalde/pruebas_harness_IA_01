# Lista de tests: estructura básica del edificio y ascensores

Los requisitos y el modelo temporal de referencia están en `trabajo/2_en_curso/lista_de_requisitos_y_casos_de_uso.md`; véase sobre todo el apartado R6 y su tabla de ejemplo.

Salvo que el nombre del test diga otra cosa, los tests usan la configuración estándar: plantas de la -2 a la 7, 3 ascensores, tiempo de desplazamiento de 2 s y tiempo de arranque y de parada de 4 s.

Ubicación sugerida: un módulo `#[cfg(test)] mod tests` en el archivo de cada tipo (`planta.rs`, `configuracion.rs`, `simulador.rs`...).

## Plantas y distancia

- [x] fn distancia_de_una_planta_a_si_misma_es_cero()
- [x] fn distancia_entre_dos_plantas_es_el_valor_absoluto_de_la_diferencia_de_sus_identificadores()
- [x] fn distancia_entre_dos_plantas_es_simetrica()
- [x] fn distancia_entre_planta_por_debajo_y_planta_por_encima_de_la_principal_suma_ambos_tramos()

## Configuración del edificio

- [x] fn configuracion_estandar_tiene_10_plantas_desde_la_menos_2_hasta_la_7()
- [x] fn configuracion_estandar_tiene_3_ascensores()
- [x] fn configuracion_estandar_tiene_tiempo_de_desplazamiento_de_2_segundos_y_de_arranque_y_de_parada_de_4_segundos()
- [x] fn duracion_de_un_desplazamiento_es_distancia_por_tiempo_de_desplazamiento_mas_tiempo_de_arranque_y_de_parada()
- [x] fn crear_simulador_con_planta_mas_baja_por_encima_de_la_mas_alta_da_error_de_configuracion()
- [x] fn crear_simulador_sin_la_planta_principal_en_el_rango_de_plantas_da_error_de_configuracion()
- [x] fn crear_simulador_sin_ascensores_da_error_de_configuracion()
- [x] fn crear_simulador_con_tiempo_de_desplazamiento_cero_da_error_de_configuracion()
- [x] fn crear_simulador_con_tiempo_de_arranque_y_de_parada_cero_es_valido()

## Estado inicial y consulta de posiciones

- [x] fn al_iniciar_la_simulacion_el_instante_actual_es_cero()
- [x] fn al_iniciar_la_simulacion_todos_los_ascensores_estan_parados_en_la_planta_principal()
- [x] fn posiciones_de_todos_los_ascensores_devuelve_una_por_ascensor_ordenadas_por_identificador()
- [x] fn consultar_posicion_de_ascensor_con_identificador_cero_da_error_de_ascensor_inexistente()
- [x] fn consultar_posicion_de_ascensor_con_identificador_mayor_que_el_numero_de_ascensores_da_error_de_ascensor_inexistente()

## Avance del tiempo simulado

- [x] fn avanzar_tiempo_suma_la_duracion_al_instante_actual()
- [x] fn avanzar_tiempo_cero_no_cambia_nada()
- [x] fn avanzar_tiempo_sin_ordenes_no_mueve_ningun_ascensor()

## Orden de mover un ascensor: validaciones

- [x] fn ordenar_mover_ascensor_inexistente_da_error_de_ascensor_inexistente()
- [x] fn ordenar_mover_ascensor_a_planta_por_encima_de_la_mas_alta_da_error_de_planta_inexistente()
- [x] fn ordenar_mover_ascensor_a_planta_por_debajo_de_la_mas_baja_da_error_de_planta_inexistente()
- [x] fn ordenar_mover_ascensor_a_la_planta_mas_alta_es_aceptado()
- [x] fn ordenar_mover_ascensor_a_la_planta_mas_baja_es_aceptado()
- [x] fn ordenar_mover_ascensor_desplazandose_da_error_de_ascensor_ocupado()
- [x] fn ordenar_mover_ascensor_durante_su_parada_en_la_planta_de_destino_da_error_de_ascensor_ocupado()
- [x] fn una_orden_rechazada_no_altera_la_posicion_ni_el_estado_del_ascensor()
- [x] fn ordenar_mover_ascensor_a_la_planta_en_la_que_esta_parado_lo_deja_parado_en_ella()

## Desplazamiento de un ascensor en el tiempo

- [x] fn tras_ordenar_mover_un_ascensor_se_esta_desplazando_hacia_la_planta_de_destino_sin_haber_salido_de_la_de_origen()
- [x] fn durante_el_arranque_el_ascensor_permanece_en_la_planta_de_origen()
- [x] fn el_ascensor_alcanza_cada_planta_intermedia_al_cumplirse_su_tiempo_de_desplazamiento()
- [x] fn entre_dos_plantas_la_planta_actual_es_la_ultima_alcanzada()
- [x] fn durante_la_parada_el_ascensor_esta_en_la_planta_de_destino_pero_sigue_desplazandose()
- [x] fn al_completar_el_desplazamiento_el_ascensor_queda_parado_en_la_planta_de_destino()
- [x] fn el_ascensor_baja_planta_a_planta_hasta_una_planta_por_debajo_de_la_principal()
- [x] fn con_tiempo_de_arranque_y_de_parada_cero_el_ascensor_queda_parado_al_alcanzar_la_planta_de_destino()
- [x] fn avanzar_tiempo_admite_fracciones_de_segundo()
- [x] fn avanzar_tiempo_de_una_vez_mas_alla_del_final_deja_el_ascensor_parado_en_la_planta_de_destino()
- [x] fn avanzar_tiempo_en_varios_pasos_da_la_misma_posicion_que_avanzarlo_de_una_vez()
- [x] fn el_desplazamiento_se_cuenta_desde_el_instante_en_que_se_ordena()
- [x] fn un_ascensor_que_ha_completado_su_desplazamiento_puede_desplazarse_de_nuevo_desde_la_planta_alcanzada()

## Independencia entre ascensores

- [x] fn mover_un_ascensor_no_altera_la_posicion_de_los_demas()
- [x] fn varios_ascensores_se_desplazan_a_la_vez_de_forma_independiente()
- [x] fn ascensores_que_reciben_la_orden_en_instantes_distintos_avanzan_cada_uno_segun_su_propio_inicio()
