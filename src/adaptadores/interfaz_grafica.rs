//! Interfaz grafica de usuario: adaptador de entrada que maneja el control de trafico.
//! Sigue el patron modelo-vista-presentador de vista pasiva: el presentador decide que se
//! muestra y la vista con egui solo dibuja y recoge las pulsaciones.

pub mod modelo_de_vista;
pub mod presentador;
pub mod vista_con_egui;
