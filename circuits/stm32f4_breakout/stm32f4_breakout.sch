EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L MCU_ST_STM32F4:STM32F405RGTx U?
U 1 1 61162DD6
P 5900 3850
F 0 "U?" H 5900 1961 50  0000 C CNN
F 1 "STM32F405RGTx" H 5900 1870 50  0000 C CNN
F 2 "Package_QFP:LQFP-64_10x10mm_P0.5mm" H 5300 2150 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00037051.pdf" H 5900 3850 50  0001 C CNN
	1    5900 3850
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Linear:AMS1117-3.3 U?
U 1 1 61164DEE
P 2800 1550
F 0 "U?" H 2800 1792 50  0000 C CNN
F 1 "AMS1117-3.3" H 2800 1701 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 2800 1750 50  0001 C CNN
F 3 "http://www.advanced-monolithic.com/pdf/ds1117.pdf" H 2900 1300 50  0001 C CNN
	1    2800 1550
	1    0    0    -1  
$EndComp
$Comp
L Device:Ferrite_Bead_Small FB?
U 1 1 6116775D
P 2100 1550
F 0 "FB?" V 1863 1550 50  0000 C CNN
F 1 "100 @ 100 MHz" V 1954 1550 50  0000 C CNN
F 2 "" V 2030 1550 50  0001 C CNN
F 3 "~" H 2100 1550 50  0001 C CNN
	1    2100 1550
	0    1    1    0   
$EndComp
$Comp
L Device:Fuse_Small F?
U 1 1 61169972
P 1600 1550
F 0 "F?" H 1600 1735 50  0000 C CNN
F 1 "500mA" H 1600 1644 50  0000 C CNN
F 2 "" H 1600 1550 50  0001 C CNN
F 3 "~" H 1600 1550 50  0001 C CNN
	1    1600 1550
	1    0    0    -1  
$EndComp
$Comp
L Device:D_Schottky D?
U 1 1 6116A9CB
P 1050 1550
F 0 "D?" H 1050 1333 50  0000 C CNN
F 1 "B5B19W" H 1050 1424 50  0000 C CNN
F 2 "" H 1050 1550 50  0001 C CNN
F 3 "~" H 1050 1550 50  0001 C CNN
	1    1050 1550
	-1   0    0    1   
$EndComp
Wire Wire Line
	1200 1550 1500 1550
Wire Wire Line
	1700 1550 2000 1550
Wire Wire Line
	2200 1550 2350 1550
$Comp
L Device:C C?
U 1 1 61188E13
P 2350 1750
F 0 "C?" H 2465 1796 50  0000 L CNN
F 1 "C" H 2465 1705 50  0000 L CNN
F 2 "" H 2388 1600 50  0001 C CNN
F 3 "~" H 2350 1750 50  0001 C CNN
	1    2350 1750
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 6118953F
P 3250 1750
F 0 "C?" H 3365 1796 50  0000 L CNN
F 1 "C" H 3365 1705 50  0000 L CNN
F 2 "" H 3288 1600 50  0001 C CNN
F 3 "~" H 3250 1750 50  0001 C CNN
	1    3250 1750
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 61189E73
P 2800 1950
F 0 "#PWR?" H 2800 1700 50  0001 C CNN
F 1 "GND" H 2805 1777 50  0000 C CNN
F 2 "" H 2800 1950 50  0001 C CNN
F 3 "" H 2800 1950 50  0001 C CNN
	1    2800 1950
	1    0    0    -1  
$EndComp
Wire Wire Line
	2350 1600 2350 1550
Connection ~ 2350 1550
Wire Wire Line
	2350 1550 2500 1550
Wire Wire Line
	2350 1900 2350 1950
Wire Wire Line
	2350 1950 2800 1950
Wire Wire Line
	3250 1900 3250 1950
Wire Wire Line
	3250 1950 2800 1950
Connection ~ 2800 1950
Wire Wire Line
	2800 1850 2800 1950
Wire Wire Line
	3100 1550 3250 1550
Wire Wire Line
	3250 1550 3250 1600
$Comp
L power:+3.3V #PWR?
U 1 1 61193663
P 3250 1550
F 0 "#PWR?" H 3250 1400 50  0001 C CNN
F 1 "+3.3V" H 3265 1723 50  0000 C CNN
F 2 "" H 3250 1550 50  0001 C CNN
F 3 "" H 3250 1550 50  0001 C CNN
	1    3250 1550
	1    0    0    -1  
$EndComp
Connection ~ 3250 1550
Wire Notes Line
	800  1250 800  2200
Wire Notes Line
	800  2200 3500 2200
Wire Notes Line
	3500 2200 3500 1250
Wire Notes Line
	3500 1250 800  1250
Text Notes 800  1200 0    50   ~ 0
Input Voltage Regulator
$EndSCHEMATC
