# Lista de tests: control de tráfico básico

Los requisitos, los supuestos (T1–T18) y los escenarios de referencia están en `trabajo/2_en_curso/lista_de_requisitos_y_casos_de_uso.md`. Conviene leer sobre todo los apartados R6, R7 y 5 (ejemplos de referencia).

Salvo que el nombre del test diga otra cosa, los tests usan:
- el edificio estándar: plantas de la -2 a la 7, 3 ascensores, 2 s por planta, y 4 s de arranque y de parada;
- la configuración estándar del control de tráfico: 28 días de histórico y 30 s de reposo;
- como fecha y hora de inicio, el lunes 5 de octubre de 2026 a las 08:00:00;
- el histórico en memoria.

Ubicación sugerida: un módulo `#[cfg(test)] mod tests` en el archivo de cada tipo. Los encabezados indican el archivo.

## Fecha y hora (`src/dominio/fecha_y_hora.rs`)

- [x] fn dia_de_la_semana_de_fechas_conocidas_es_el_correcto()
- [x] fn hora_del_dia_es_la_hora_sin_minutos_ni_segundos()
- [x] fn sumar_una_duracion_avanza_la_fecha_y_hora_esa_duracion()
- [x] fn sumar_una_duracion_que_pasa_de_medianoche_cambia_de_dia_y_de_dia_de_la_semana()
- [x] fn restar_dias_retrocede_ese_numero_de_dias_conservando_la_hora()
- [x] fn crear_una_fecha_y_hora_imposible_no_da_fecha_y_hora()
- [x] fn fecha_y_hora_se_escribe_como_texto_iso_8601()
- [x] fn fecha_y_hora_escrita_como_texto_y_leida_de_nuevo_es_la_misma_incluso_con_fraccion_de_segundo()
- [x] fn leer_una_fecha_y_hora_de_un_texto_mal_formado_da_error()

## Histórico en memoria (`src/adaptadores/historico_de_movimientos_en_memoria.rs`)

- [x] fn un_historico_en_memoria_nuevo_esta_vacio()
- [x] fn los_movimientos_registrados_en_memoria_se_recuperan_en_el_orden_en_que_se_registraron()
- [x] fn movimientos_desde_una_fecha_y_hora_excluye_los_anteriores_e_incluye_los_de_esa_misma_fecha_y_hora()

## Selección del ascensor libre más cercano (`src/dominio/seleccion_de_ascensor.rs`)

- [x] fn sin_ascensores_parados_no_hay_ascensor_libre_mas_cercano()
- [x] fn el_ascensor_libre_mas_cercano_es_el_parado_a_menor_distancia_de_la_planta_de_la_llamada()
- [x] fn un_ascensor_parado_en_la_planta_de_la_llamada_es_el_libre_mas_cercano()
- [x] fn los_ascensores_que_se_estan_desplazando_no_cuentan_como_libres_aunque_esten_mas_cerca()
- [x] fn a_igual_distancia_se_escoge_el_ascensor_libre_de_menor_identificador()

## Control de tráfico: creación y tiempo (`src/aplicacion/control_de_trafico.rs`)

- [x] fn configuracion_estandar_del_control_de_trafico_considera_28_dias_de_historico_y_30_segundos_de_reposo()
- [x] fn al_crear_el_control_de_trafico_no_hay_llamadas_pendientes_y_la_fecha_y_hora_actual_es_la_de_inicio()
- [x] fn avanzar_tiempo_en_el_control_avanza_el_instante_del_simulador_y_la_fecha_y_hora_actual()

## Control de tráfico: llamadas (`src/aplicacion/control_de_trafico.rs`)

- [x] fn pulsar_el_boton_de_llamada_de_una_planta_inexistente_da_error_de_planta_inexistente()
- [x] fn una_llamada_envia_el_ascensor_libre_mas_cercano_a_la_planta_de_la_llamada()
- [x] fn una_llamada_desde_una_planta_con_un_ascensor_parado_en_ella_se_atiende_con_ese_ascensor_sin_moverlo()
- [x] fn una_llamada_no_envia_ascensores_que_se_estan_desplazando()
- [x] fn una_llamada_sin_ascensores_libres_queda_pendiente()
- [x] fn una_llamada_pendiente_se_atiende_al_avanzar_el_tiempo_en_cuanto_queda_libre_un_ascensor()
- [x] fn las_llamadas_pendientes_se_atienden_por_orden_de_llegada()
- [x] fn pulsar_de_nuevo_el_boton_de_una_planta_con_llamada_pendiente_no_crea_otra_llamada()
- [x] fn pulsar_el_boton_de_una_planta_hacia_la_que_ya_se_desplaza_un_ascensor_no_envia_otro()

## Control de tráfico: botonera (`src/aplicacion/control_de_trafico.rs`)

- [x] fn pulsar_un_boton_de_la_botonera_envia_ese_ascensor_a_la_planta_indicada()
- [x] fn pulsar_un_boton_de_la_botonera_de_un_ascensor_inexistente_da_error_de_ascensor_inexistente()
- [x] fn pulsar_un_boton_de_la_botonera_hacia_una_planta_inexistente_da_error_de_planta_inexistente()
- [x] fn pulsar_un_boton_de_la_botonera_de_un_ascensor_que_se_desplaza_da_error_de_ascensor_ocupado()

## Control de tráfico: registro en el histórico (`src/aplicacion/control_de_trafico.rs`)

- [x] fn atender_una_llamada_registra_un_movimiento_por_llamada_con_su_fecha_y_hora_ascensor_origen_y_destino()
- [x] fn atender_una_llamada_con_un_ascensor_ya_en_la_planta_registra_un_movimiento_con_origen_igual_a_destino()
- [x] fn una_llamada_pendiente_se_registra_al_atenderse_con_la_fecha_y_hora_en_que_se_atiende()
- [x] fn una_llamada_repetida_que_no_envia_ascensor_no_registra_movimiento()
- [x] fn una_orden_de_la_botonera_registra_un_movimiento_por_botonera()
- [x] fn una_orden_de_la_botonera_a_la_planta_en_la_que_esta_parado_el_ascensor_no_registra_movimiento()
- [x] fn una_orden_rechazada_no_registra_movimiento()
- [x] fn si_el_historico_falla_al_registrar_la_operacion_devuelve_error_de_historico()

## Histórico en archivo (`src/adaptadores/historico_de_movimientos_en_archivo.rs`)

- [x] fn abrir_un_historico_en_archivo_inexistente_da_un_historico_vacio()
- [x] fn el_archivo_del_historico_tiene_una_cabecera_y_una_linea_de_texto_por_movimiento()
- [x] fn los_movimientos_registrados_en_archivo_se_recuperan_iguales_al_reabrir_el_archivo()
- [x] fn registrar_en_archivo_anade_al_final_sin_borrar_los_movimientos_de_sesiones_anteriores()
- [x] fn movimientos_desde_en_un_historico_en_archivo_excluye_los_anteriores_a_esa_fecha_y_hora()
- [x] fn registrar_en_archivo_crea_la_carpeta_si_no_existe()
- [x] fn abrir_un_historico_en_archivo_con_una_linea_ilegible_da_error_con_el_numero_de_linea()

## Plantas de espera preferentes (`src/dominio/reposicionamiento.rs`)

- [x] fn sin_movimientos_no_hay_plantas_de_espera_preferentes()
- [x] fn solo_cuentan_como_demanda_los_movimientos_por_llamada()
- [x] fn solo_cuentan_las_llamadas_del_mismo_dia_de_la_semana()
- [x] fn solo_cuentan_las_llamadas_de_la_misma_franja_horaria()
- [x] fn la_planta_que_recibe_la_demanda_es_la_de_la_llamada_y_no_la_de_origen_del_ascensor()
- [x] fn las_llamadas_del_mismo_dia_y_franja_de_semanas_distintas_se_suman()
- [x] fn las_plantas_de_espera_preferentes_se_ordenan_de_mas_a_menos_llamadas()
- [x] fn a_igual_numero_de_llamadas_se_prefiere_la_planta_mas_cercana_a_la_principal()
- [x] fn a_igual_numero_de_llamadas_y_de_distancia_a_la_principal_se_prefiere_la_planta_de_menor_identificador()
- [x] fn las_llamadas_a_plantas_que_no_existen_en_el_edificio_se_ignoran()

## Plan de reposicionamiento (`src/dominio/reposicionamiento.rs`)

- [x] fn sin_plantas_de_espera_preferentes_ningun_ascensor_se_reposiciona()
- [x] fn sin_ascensores_en_reposo_no_hay_reposicionamientos()
- [x] fn un_ascensor_en_reposo_va_a_la_planta_de_espera_de_mas_prioridad()
- [x] fn con_menos_ascensores_en_reposo_que_plantas_de_espera_solo_se_cubren_las_de_mas_prioridad()
- [x] fn con_mas_ascensores_en_reposo_que_plantas_de_espera_los_sobrantes_se_quedan_donde_estan()
- [x] fn los_ascensores_que_ya_estan_en_una_planta_a_cubrir_se_quedan_y_los_demas_cubren_las_restantes()
- [x] fn cada_planta_a_cubrir_se_asigna_al_ascensor_en_reposo_disponible_mas_cercano()
- [x] fn a_igual_distancia_se_reposiciona_el_ascensor_de_menor_identificador()
- [x] fn si_varios_ascensores_en_reposo_estan_en_la_misma_planta_a_cubrir_solo_se_queda_el_de_menor_identificador()
- [x] fn las_plantas_cubiertas_por_otros_ascensores_no_se_vuelven_a_cubrir()

## Control de tráfico: reposicionamiento (`src/aplicacion/control_de_trafico.rs`)

- [x] fn sin_historico_los_ascensores_libres_no_se_reposicionan()
- [x] fn los_ascensores_en_reposo_se_reposicionan_a_las_plantas_con_mas_llamadas_del_mismo_dia_de_la_semana_y_franja_horaria()
- [x] fn un_ascensor_libre_no_se_reposiciona_hasta_cumplir_el_tiempo_de_reposo()
- [x] fn el_tiempo_de_reposo_se_cuenta_desde_el_final_del_ultimo_desplazamiento_ordenado()
- [x] fn las_llamadas_de_hace_mas_de_28_dias_no_influyen_en_el_reposicionamiento()
- [x] fn al_cambiar_de_franja_horaria_los_ascensores_en_reposo_se_reposicionan_segun_la_nueva_franja()
- [x] fn un_reposicionamiento_registra_un_movimiento_por_reposicionamiento()
- [x] fn las_llamadas_pendientes_se_atienden_antes_de_reposicionar_ascensores()
- [x] fn un_ascensor_en_reposo_no_se_reposiciona_hacia_una_planta_a_la_que_ya_se_dirige_otro_ascensor()
