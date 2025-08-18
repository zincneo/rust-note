extends Node

func _init() -> void:
	pass

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	Singleton.install_bevy()
	pass # Replace with function body.
	
func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.is_pressed():
		if event.keycode == KEY_Q:
			Singleton.uninstall_bevy()
