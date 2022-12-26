EESchema Schematic File Version 4
LIBS:ButterStick-cache
EELAYER 29 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 7 15
Title "ButterStick"
Date "2019-06-14"
Rev "r0.2"
Comp "GsD"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L gkl_lattice:ECP5U25-BG381 U?
U 4 1 5C810715
P 3650 2450
AR Path="/5AB8ACB7/5C810715" Ref="U?"  Part="4" 
AR Path="/5C80F19D/5C810715" Ref="U3"  Part="4" 
F 0 "U3" H 4600 2600 60  0000 L CNN
F 1 "ECP5U25" H 3850 2600 60  0000 L CNN
F 2 "gkl_housings_bga:caBGA_381_17x17" H 3650 2450 50  0001 C CNN
F 3 "" H 3650 2450 50  0001 C CNN
F 4 " 220-2052-ND " H -200 -1950 50  0001 C CNN "SN-DK"
F 5 "LFE5U-45F-8BG381C" H -200 -1950 50  0001 C CNN "PN"
F 6 "Lattice" H -200 -1950 50  0001 C CNN "Mfg"
	4    3650 2450
	-1   0    0    -1  
$EndComp
Text Notes 2500 2250 0    50   ~ 0
BANK3 - 1V8
$Sheet
S 6500 2200 1650 3400
U 5C8A3B67
F0 "sheet5C8A3B42" 50
F1 "SyzygyStandard.sch" 50
F2 "S16" I L 6500 2300 50 
F3 "S17" I L 6500 2400 50 
F4 "S18" I L 6500 2500 50 
F5 "S19" I L 6500 2600 50 
F6 "S20" I L 6500 2700 50 
F7 "S21" I L 6500 2800 50 
F8 "S22" I L 6500 2900 50 
F9 "S23" I L 6500 3000 50 
F10 "S24" I L 6500 3100 50 
F11 "S25" I L 6500 3200 50 
F12 "S26" I L 6500 3300 50 
F13 "S27" I L 6500 3400 50 
F14 "D0p" I L 6500 3500 50 
F15 "D0n" I L 6500 3600 50 
F16 "D1p" I L 6500 3700 50 
F17 "D1n" I L 6500 3800 50 
F18 "D2p" I L 6500 3900 50 
F19 "D2n" I L 6500 4000 50 
F20 "D3p" I L 6500 4100 50 
F21 "D3n" I L 6500 4200 50 
F22 "D4p" I L 6500 4300 50 
F23 "D4n" I L 6500 4400 50 
F24 "D5p" I L 6500 4500 50 
F25 "D5n" I L 6500 4600 50 
F26 "D6p" I L 6500 4700 50 
F27 "D6n" I L 6500 4800 50 
F28 "D7p" I L 6500 4900 50 
F29 "D7n" I L 6500 5000 50 
F30 "P2C_CLKp" I L 6500 5200 50 
F31 "P2C_CLKn" I L 6500 5300 50 
F32 "C2P_CLKp" I L 6500 5400 50 
F33 "C2P_CLKn" I L 6500 5500 50 
F34 "SCL" I R 8150 2500 50 
F35 "SDA" I R 8150 2600 50 
F36 "PMIC_SCL" I R 8150 2800 50 
F37 "PMIC_SDA" I R 8150 2900 50 
F38 "PMIC_ADR0" I R 8150 3100 50 
F39 "VCCIO" I R 8150 2300 50 
F40 "R_GA" I R 8150 3200 50 
F41 "PMIC_EN" I R 8150 2400 50 
$EndSheet
Wire Wire Line
	6500 2300 6050 2300
Wire Wire Line
	6500 2400 6050 2400
Wire Wire Line
	6500 2500 6050 2500
Wire Wire Line
	6500 2600 6050 2600
Wire Wire Line
	6500 2700 6050 2700
Wire Wire Line
	6500 2800 6050 2800
Wire Wire Line
	6500 2900 6050 2900
Wire Wire Line
	6500 3000 6050 3000
Wire Wire Line
	6500 3100 6050 3100
Wire Wire Line
	6500 3200 6050 3200
Wire Wire Line
	6500 3300 6050 3300
Wire Wire Line
	6500 3400 6050 3400
Wire Wire Line
	6500 3500 5950 3500
Wire Wire Line
	6500 3600 5950 3600
Wire Wire Line
	6500 3700 5950 3700
Wire Wire Line
	6500 3800 5950 3800
Wire Wire Line
	6500 3900 5950 3900
Wire Wire Line
	6500 4000 5950 4000
Wire Wire Line
	6500 4100 5950 4100
Wire Wire Line
	6500 4200 5950 4200
Wire Wire Line
	6500 4300 5950 4300
Wire Wire Line
	6500 4400 5950 4400
Wire Wire Line
	6500 4500 5950 4500
Wire Wire Line
	6500 4600 5950 4600
Wire Wire Line
	6500 4700 5950 4700
Wire Wire Line
	6500 4800 5950 4800
Wire Wire Line
	6500 4900 5950 4900
Wire Wire Line
	6500 5000 5950 5000
Wire Wire Line
	6500 5200 5850 5200
Wire Wire Line
	6500 5300 5850 5300
Wire Wire Line
	6500 5400 5850 5400
Wire Wire Line
	6500 5500 5850 5500
Text Label 6050 2300 0    50   ~ 0
S16
Text Label 6050 2400 0    50   ~ 0
S17
Text Label 6050 2500 0    50   ~ 0
S18
Text Label 6050 2600 0    50   ~ 0
S19
Text Label 6050 2700 0    50   ~ 0
S20
Text Label 6050 2800 0    50   ~ 0
S21
Text Label 6050 2900 0    50   ~ 0
S22
Text Label 6050 3000 0    50   ~ 0
S23
Text Label 6050 3100 0    50   ~ 0
S24
Text Label 6050 3200 0    50   ~ 0
S25
Text Label 6050 3300 0    50   ~ 0
S26
Text Label 6050 3400 0    50   ~ 0
S27
Text Label 5950 3500 0    50   ~ 0
D0_P
Text Label 5950 3600 0    50   ~ 0
D0_N
Text Label 5950 3700 0    50   ~ 0
D1_P
Text Label 5950 3800 0    50   ~ 0
D1_N
Text Label 5950 3900 0    50   ~ 0
D2_P
Text Label 5950 4000 0    50   ~ 0
D2_N
Text Label 5950 4100 0    50   ~ 0
D3_P
Text Label 5950 4200 0    50   ~ 0
D3_N
Text Label 5950 4300 0    50   ~ 0
D4_P
Text Label 5950 4400 0    50   ~ 0
D4_N
Text Label 5950 4500 0    50   ~ 0
D5_P
Text Label 5950 4600 0    50   ~ 0
D5_N
Text Label 5950 4700 0    50   ~ 0
D6_P
Text Label 5950 4800 0    50   ~ 0
D6_N
Text Label 5950 4900 0    50   ~ 0
D7_P
Text Label 5950 5000 0    50   ~ 0
D7_N
Text Label 5850 5200 0    50   ~ 0
P2C_CLK_P
Text Label 5850 5300 0    50   ~ 0
P2C_CLK_N
Text Label 5850 5400 0    50   ~ 0
C2P_CLK_P
Text Label 5850 5500 0    50   ~ 0
C2P_CLK_N
Wire Wire Line
	3650 4350 4100 4350
Wire Wire Line
	3650 2850 4100 2850
Wire Wire Line
	3650 4250 4100 4250
Wire Wire Line
	3650 3850 4100 3850
Wire Wire Line
	3650 3050 4100 3050
Wire Wire Line
	3650 2950 4100 2950
Wire Wire Line
	3650 4750 4100 4750
Wire Wire Line
	3650 3150 4100 3150
Wire Wire Line
	3650 4650 4100 4650
Wire Wire Line
	3650 3950 4100 3950
Wire Wire Line
	3650 3450 4100 3450
Wire Wire Line
	3650 3550 4100 3550
Wire Wire Line
	3650 3250 4300 3250
Wire Wire Line
	3650 3350 4300 3350
Wire Wire Line
	3650 5450 4300 5450
Wire Wire Line
	3650 5550 4300 5550
Wire Wire Line
	3650 3650 4300 3650
Wire Wire Line
	3650 3750 4300 3750
Wire Wire Line
	3650 5250 4300 5250
Wire Wire Line
	3650 5350 4300 5350
Wire Wire Line
	3650 4450 4300 4450
Wire Wire Line
	3650 4550 4300 4550
Wire Wire Line
	3650 5050 4300 5050
Wire Wire Line
	3650 5150 4300 5150
Wire Wire Line
	3650 4050 4300 4050
Wire Wire Line
	3650 4150 4300 4150
Wire Wire Line
	3650 4850 4300 4850
Wire Wire Line
	3650 4950 4300 4950
Wire Wire Line
	3650 2650 4600 2650
Wire Wire Line
	3650 2750 4600 2750
Wire Wire Line
	3650 2450 4600 2450
Wire Wire Line
	3650 2550 4600 2550
Text Label 4100 4350 2    50   ~ 0
S16
Text Label 4100 2850 2    50   ~ 0
S17
Text Label 4100 4250 2    50   ~ 0
S18
Text Label 4100 3850 2    50   ~ 0
S19
Text Label 4100 3050 2    50   ~ 0
S20
Text Label 4100 2950 2    50   ~ 0
S21
Text Label 4100 4750 2    50   ~ 0
S22
Text Label 4100 3150 2    50   ~ 0
S23
Text Label 4100 4650 2    50   ~ 0
S24
Text Label 4100 3950 2    50   ~ 0
S25
Text Label 4100 3450 2    50   ~ 0
S26
Text Label 4100 3550 2    50   ~ 0
S27
Text Label 4300 3250 2    50   ~ 0
D0_P
Text Label 4300 3350 2    50   ~ 0
D0_N
Text Label 4300 5450 2    50   ~ 0
D1_P
Text Label 4300 5550 2    50   ~ 0
D1_N
Text Label 4300 3650 2    50   ~ 0
D2_P
Text Label 4300 3750 2    50   ~ 0
D2_N
Text Label 4300 5250 2    50   ~ 0
D3_P
Text Label 4300 5350 2    50   ~ 0
D3_N
Text Label 4300 4450 2    50   ~ 0
D4_P
Text Label 4300 4550 2    50   ~ 0
D4_N
Text Label 4300 5050 2    50   ~ 0
D5_P
Text Label 4300 5150 2    50   ~ 0
D5_N
Text Label 4300 4050 2    50   ~ 0
D6_P
Text Label 4300 4150 2    50   ~ 0
D6_N
Text Label 4300 4850 2    50   ~ 0
D7_P
Text Label 4300 4950 2    50   ~ 0
D7_N
Text Label 4600 2650 2    50   ~ 0
P2C_CLK_P
Text Label 4600 2750 2    50   ~ 0
P2C_CLK_N
Text Label 4600 2450 2    50   ~ 0
C2P_CLK_P
Text Label 4600 2550 2    50   ~ 0
C2P_CLK_N
Text HLabel 8550 2300 2    50   Input ~ 0
VCCIO
Wire Wire Line
	8150 3200 8400 3200
Wire Wire Line
	8400 3200 8400 3500
$Comp
L Device:R R?
U 1 1 5C996E36
P 8400 3650
AR Path="/5C80F1A0/5C996E36" Ref="R?"  Part="1" 
AR Path="/5C80F19D/5C996E36" Ref="R26"  Part="1" 
F 0 "R26" H 8470 3696 50  0000 L CNN
F 1 "84k5" H 8470 3605 50  0000 L CNN
F 2 "pkl_dipol:R_0402" V 8330 3650 50  0001 C CNN
F 3 "~" H 8400 3650 50  0001 C CNN
F 4 "Stackpole Electronics Inc" H 0   0   50  0001 C CNN "Mfg"
F 5 "RMCF0402FT84K5" H 0   0   50  0001 C CNN "PN"
F 6 "1%" H 0   0   50  0001 C CNN "Tol"
	1    8400 3650
	1    0    0    -1  
$EndComp
$Comp
L gkl_power:GND #PWR?
U 1 1 5C996E3D
P 8400 3800
AR Path="/5C80F1A0/5C996E3D" Ref="#PWR?"  Part="1" 
AR Path="/5C80F19D/5C996E3D" Ref="#PWR0214"  Part="1" 
F 0 "#PWR0214" H 8400 3550 50  0001 C CNN
F 1 "GND" H 8403 3674 50  0000 C CNN
F 2 "" H 8300 3450 50  0001 C CNN
F 3 "" H 8400 3800 50  0001 C CNN
	1    8400 3800
	1    0    0    -1  
$EndComp
$Comp
L gkl_power:GND #PWR?
U 1 1 5C996E43
P 8800 3100
AR Path="/5C80F1A0/5C996E43" Ref="#PWR?"  Part="1" 
AR Path="/5C80F19D/5C996E43" Ref="#PWR0215"  Part="1" 
F 0 "#PWR0215" H 8800 2850 50  0001 C CNN
F 1 "GND" V 8804 3020 50  0000 R CNN
F 2 "" H 8700 2750 50  0001 C CNN
F 3 "" H 8800 3100 50  0001 C CNN
	1    8800 3100
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8150 3100 8800 3100
Wire Wire Line
	8150 2300 8550 2300
Wire Wire Line
	8150 2500 8550 2500
Wire Wire Line
	8150 2600 8550 2600
Wire Wire Line
	8150 2800 8550 2800
Wire Wire Line
	8150 2900 8550 2900
Text HLabel 8550 2500 2    50   Input ~ 0
syzygySCL
Text HLabel 8550 2600 2    50   Input ~ 0
syzygySDA
Text HLabel 8550 2800 2    50   Input ~ 0
pmicSCL
Text HLabel 8550 2900 2    50   Input ~ 0
pmicSDA
Wire Wire Line
	8150 2400 8550 2400
Text HLabel 8550 2400 2    50   Input ~ 0
EN
$EndSCHEMATC
