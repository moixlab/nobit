# Método Nobit para el registro de información

```haskell
Observador <- [Observable]

Observable <- [Registro]

Registro <- {
  Identidad, 
  Tiempo, 
  Inmutable <- {
    Concepto, 
    Naturaleza <- (
      Debit | +1, 
      Credit | -1, 
      Nobit | 0
    ), 
    Cantidad <- (Unidad de medida > 0)
  }, 
  Mutable <- {
    Agrupación, 
    Referencia,
    Detalles
  }
}

```

## Generalidades

* Datos. Datos sin categorización.
* Datos inmutables. Datos que son registro de eventos pasados o de gran relevancia.
* Datos mutables. Datos que son necesarios para dar contexto a los datos inmutables.
* Observador. Persona o personas que hacen uso de los datos.
* Observable. Objeto o ente que realiza los eventos o genera los datos a registrar.
* Unidad de medida. Valor con el cual se puede registrar los datos inmutables y no permite cero o números negativos.

## Generación de datos

### El observador

El observador por el simple hecho de estar predispuesto a registrar lo observado, sirve como filtro y también como primer factor de distorsión lo observado y registrado.

### Eventos CAOS

CAOS, es el orden en que un evento es captado por el observador y tenemos:

1. Consecuencias. Son los efectos de la realización del evento.
2. Acciones. Es el evento en ejecutado.
3. Objeto. Es hacia donde va dirigido el evento.
4. Sujeto. Es el que realiza el evento.

## Estructura para el registro de datos

### Identidad

Identidad, es lo que distingue al registro de entre otros y no es repetible.

### Tiempo

El tiempo por su carácter relativo es un conteo cíclico.

### Naturaleza

Base fundamental del método, donde categoriza los datos contables en:

* Debit (+1). Datos expresados en valor de la unidad de medida que son favorables de lo observable.
* Credit (-1). Datos expresado en valor de la unidad de medida que son desfavorables a lo observable.
* Nobit (0). Datos que no se pueden expresar en la unidad de medida y son relevantes para el observador.

### Inmutable

Es el registro de datos que tienen una justificación y se pueden expresar en la unidad de medida, para lo cual se requiere:

* Concepto. Es la categoría o identificador de la misma a lo cual pertenece el registro (cuanta contable).
* Naturaleza. Se especifica si los datos son positivos, negativos o neutrales.
* Cantidad. Cuantificación del concepto usando la unidad de medida (no acepta cero o negativos).

### Mutable

Es el registro de datos que permiten dar contexto a los inmutables, de forma opcional pueden ser:

* Agrupación. Se especifica si los datos pertenecen a un colección o grupo.
* Referencia. Se especifica si los datos guardan relación a otros registros.
* Detalles. Contexto para entender los datos.

## Ejemplos de conceptualización de un evento

> Los conceptos pueden estar registrados como datos relevantes y parte de una agrupación, o pueden ser conceptos implícitos a la actividad registrada.

### Pelota rebota

1. Observador: Persona.
2. Observable: Pelota.
3. Conceptos: a) Pelota no rebota, b) Pelota rebota, c) Pelota golpea superficie.
4. Registro:

| Id | Tiempo | Concepto | Naturaleza | Cantidad | Referencia (Id)
| -- | --   | --        | --        | -- | --
| 1 | 1.1 | c | 0 | 1 | 
| 2 | 10.5 | b | 0 | 1 | 1
| 3 | 20.7 | a | 0 | 1 | 1

### Cheque rebota

1. Observador: Tesorero.
2. Observable: Cheque de $100.
3. Conceptos: a) Cheque no aceptado, b) Cheque aceptado, c) Cheque incobrable.
4. Registro:

| Id | Tiempo | Concepto | Naturaleza | Cantidad | Colección (Id) | Referencia (Id) | Detalles
| -- | --   | --        | --        | --        | --        | --        | --
| 1 | 1 Ene | Conceptos | 0 | 1 |
| 2 | 1 Ene | a | 0 | 1 | 1 | | Cheque no aceptado 
| 3 | 1 Ene | b | 0 | 1 | 1 | | Cheque aceptado
| 4 | 1 Ene | c | 0 | 1 | 1 | | Cheque incobrable
| ... | | | | | | | ...
| 11 | 1 Ene | Cheques | 0 | 1 | | | Agrupación de cheques
| 12 | 7 Ene | b | 1 | 100.00 | 11 | | Cliente paga con cheque
| 13 | 8 Ene | a | 0 | 1 | 11 | 12 | Banco no acepta el cheque
| 14 | 15 Ene | c | -1 | 100.00 | 11 | 12 | Cheque anulado; fue pagado por cliente en efectivo
