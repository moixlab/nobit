## Estructura de nombres de los archivos

| Posición | Nemotécnico | Descripción                                                                                                                                                  |
| -------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 01-02    | LE          | Identificador fijo 'LE'                                                                                                                                      |
| 03-13    | RRRRRRRRRRR | RUC del deudor tributario                                                                                                                                    |
| 14-17    | AAAA        | Año, aplica a todos los libros                                                                                                                               |
| 18-19    | MM          | Mes, aplica a los libros de periodicidad mensual, para los demás consigne '00':                                                                              |
|          |             | 1. Libros Caja y Bancos                                                                                                                                      |
|          |             | 2. Libro de Retenciones inciso e) y f) del Art. 34° de la LIR                                                                                                |
|          |             | 3. Libro Diario                                                                                                                                              |
|          |             | 5A. Libro Diario de Formato Simplificado                                                                                                                     |
|          |             | 4. Libro Mayor                                                                                                                                               |
|          |             | 5. Registro de Consignaciones                                                                                                                                |
|          |             | 6.Registro de Inventario Permanente en Unidades Físicas                                                                                                      |
|          |             | 7.Registro de Inventario Permanente Valorizado                                                                                                               |
|          |             | 8.Registro de Ventas e Ingresos                                                                                                                              |
| 20-21    | DD          | Día, aplica al Libro de Inventarios y Balances, para los demás consigne '00'                                                                                 |
| 22-27    | LLLLLL      | Identificador del libro                                                                                                                                      |
| 28-29    | CC          | Código de oportunidad de presentación del EEFF, aplica al Libro de Inventarios y Balances, para los demás consigne '00':                                     |
|          | 01          | Al 31 de diciembre                                                                                                                                           |
|          | 02          | Al 31 de enero, por modificación del porcentaje                                                                                                              |
|          | 03          | Al 30 de junio, por modificación del coeficiente o porcentaje                                                                                                |
|          | 04          | Al último día del mes que sustentará la suspensión o modificación del coeficiente (distinto al 31 de enero o 30 de junio)                                    |
|          | 05          | Al día anterior a la entrada en vigencia de la fusión, escisión y demás formas de reorganización de sociedades o empresas o extinción de la persona jurídica |
|          | 06          | A la fecha del balance de liquidación, cierre o cese definitivo del deudor tributario                                                                        |
|          | 07          | A la fecha de presentación para libre propósito                                                                                                              |
| 30-30    | O           | Indicador de operaciones                                                                                                                                     |
|          | 0           | Cierre de operaciones - baja de inscripción en el RUC                                                                                                        |
|          | 1           | Empresa o entidad operativa                                                                                                                                  |
|          | 2           | Cierre del libro - no obligado a llevarlo                                                                                                                    |
| 31-31    | I           | Indicador del contenido del libro o registro                                                                                                                 |
|          | 1           | Con información                                                                                                                                              |
|          | 0           | Sin información                                                                                                                                              |
| 32-32    | M           | Indicador de la moneda utilizada                                                                                                                             |
|          | 1           | Soles                                                                                                                                                        |
|          | 2           | US dólares                                                                                                                                                   |
| 33-33    | G           | Indicador de libro electrónico generado por el PLE                                                                                                           |
|          | 1           | Generado por PLE (Fijo)                                                                                                                                      |
