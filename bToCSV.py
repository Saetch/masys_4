import csv
import re

# Pfad zur Eingabedatei
eingabe_dateipfad = 'result_b.txt'

# Pfad zur Ausgabedatei
ausgabe_dateipfad = 'result_b.csv'

# Öffne die Eingabedatei zum Lesen
with open(eingabe_dateipfad, 'r') as eingabe_datei:
    # Lies die Zeilen der Datei
    zeilen = eingabe_datei.readlines()

# Öffne die Ausgabedatei zum Schreiben
with open(ausgabe_dateipfad, 'w', newline='') as ausgabe_datei:
    # Erstelle einen CSV-Writer
    csv_writer = csv.writer(ausgabe_datei)

    # Schreibe die Kopfzeile
    csv_writer.writerow(['prop_to_talk', 'average_points_TFT', 'average_points_TFT95'])

    # Iteriere über die Zeilen der Eingabedatei
    for zeile in zeilen:
        # Extrahiere die relevanten Daten mit regulären Ausdrücken
        match = re.search(r'([\d.]+):\s+([\d.]+)\s+/\s+([\d.]+)', zeile.strip())

        if match:
            prop_to_talk = round(float(match.group(1)), 3)
            average_points_TFT = round(float(match.group(2)), 3)
            average_points_TFT95 = round(float(match.group(3)), 3)

            # Schreibe die Daten in die CSV-Datei
            csv_writer.writerow([prop_to_talk, average_points_TFT, average_points_TFT95])
        else:
            print(f'Ungültige Zeile: {zeile.strip()}')

print(f'Die Daten wurden erfolgreich in "{ausgabe_dateipfad}" geschrieben.')
