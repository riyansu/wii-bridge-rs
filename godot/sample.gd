extends Node2D

func _ready():
	WiiManager.button_event.connect(_on_button_event) #ボタン通信開始
	WiiManager.pointer_event.connect(_on_pointer_event) #IR通信開始

func _on_button_event(button: String, pressed: bool):
	print("Button: ", button, ", pressed: ", pressed)

func _on_pointer_event(x: float, y: float, valid: bool):
	print("Pointer: ", x, ", ", y, " (valid: ", valid, ")")
