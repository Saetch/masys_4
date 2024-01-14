import csv
import re

# Pfad zur Eingabedatei
eingabe_dateipfad = 'result.txt'

# Pfad zur Ausgabedatei
ausgabe_dateipfad = 'result_a.csv'

# Öffne die Eingabedatei zum Lesen
with open(eingabe_dateipfad, 'r') as eingabe_datei:
    # Lies die Zeilen der Datei
    zeilen = eingabe_datei.readlines()

# Öffne die Ausgabedatei zum Schreiben
with open(ausgabe_dateipfad, 'w', newline='') as ausgabe_datei:
    # Erstelle einen CSV-Writer
    csv_writer = csv.writer(ausgabe_datei)

    # Schreibe die Kopfzeile
    csv_writer.writerow(['draw', 'total_points_TFT', 'average_points_TFT', 'total_points_TFT95', 'average_points_TFT95'])

    # Iteriere über die Zeilen der Eingabedatei in Schritten von 2
    for i in range(0, len(zeilen), 2):
        # Extrahiere die relevanten Daten mit regulären Ausdrücken
        match_zeile1 = re.search(r'(\d+) draws, average of ([\d.]+) points per game', zeilen[i])
        match_zeile2 = re.search(r'(\d+) draws, average of ([\d.]+) points per game', zeilen[i + 1])

        if match_zeile1 and match_zeile2:
            draw = match_zeile1.group(1)
            points_player1 = round(float(match_zeile1.group(2)), 3)
            average_points_player1 = round(float(match_zeile1.group(2)), 3)

            points_player6 = round(float(match_zeile2.group(2)), 3)
            average_points_player6 = round(float(match_zeile2.group(2)), 3)

            # Schreibe die Daten in die CSV-Datei
            csv_writer.writerow([draw, points_player1, average_points_player1, points_player6, average_points_player6])
        else:
            print(f'Ungültige Zeilen: {zeilen[i].strip()} {zeilen[i + 1].strip()}')

print(f'Die Daten wurden erfolgreich in "{ausgabe_dateipfad}" geschrieben.')
