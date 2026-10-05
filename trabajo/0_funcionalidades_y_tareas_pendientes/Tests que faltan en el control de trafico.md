# Tests que faltan en el control de tráfico

Origen: revisión del código de la funcionalidad "Un control de tráfico básico".

## Situación

Al revisar los tests, se aplicaron a mano dos cambios incorrectos (mutaciones) en `src/aplicacion/control_de_trafico.rs`, y los 123 tests siguieron pasando. Hay dos requisitos que ningún test comprueba:

1. **R7.2: un ascensor parado que todavía no está en reposo cubre su planta.** Si `ascensores_en_reposo_y_plantas_cubiertas_por_otros_ascensores` dejase de añadir la planta actual de esos ascensores a las plantas cubiertas, ningún test fallaría. Solo se prueba la planta de destino de los ascensores que se desplazan (`un_ascensor_en_reposo_no_se_reposiciona_hacia_una_planta_a_la_que_ya_se_dirige_otro_ascensor`).
2. **R6.6.4: si falla el registro de una llamada pendiente al avanzar el tiempo, la llamada se da por atendida** (sale de la cola de llamadas pendientes). Si se sacase de la cola después de registrarla, ningún test fallaría. `si_el_historico_falla_al_registrar_la_operacion_devuelve_error_de_historico` solo prueba `pulsar_boton_de_llamada` y `pulsar_boton_de_la_botonera`, no `avanzar_tiempo`.

## Propuesta

Añadir, en la próxima funcionalidad que toque el control de tráfico, tests como estos:

- `un_ascensor_en_reposo_no_se_reposiciona_hacia_la_planta_de_otro_ascensor_parado_que_aun_no_esta_en_reposo`. Por ejemplo, un edificio de dos ascensores y un histórico con llamadas en la 3. El ascensor 1 se manda a la 3 por la botonera. El ascensor 2 está en reposo desde el principio. Hay que comprobar que, cuando el 1 llega a la 3 y el 2 ya está en reposo, el 2 no va a la 3.
- `si_el_historico_falla_al_atender_una_llamada_pendiente_avanzar_tiempo_devuelve_error_y_la_llamada_deja_de_estar_pendiente`. Hará falta un doble de histórico que falle solo a partir del n-ésimo registro, o uno con un interruptor.
- Opcional: `pulsar_un_boton_de_la_botonera_de_un_ascensor_inexistente_hacia_una_planta_inexistente_da_error_de_ascensor_inexistente`. Comprueba en el control el orden de errores de R6.5; ahora solo se comprueba en el simulador.

## Por qué no se ha hecho en la refactorización

El paso de refactorización no puede añadir tests: `lista_de_tests.md` debe coincidir con los tests reales.
