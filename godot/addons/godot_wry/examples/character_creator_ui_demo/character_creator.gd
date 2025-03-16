extends Node3D

var rotating = false

func _input(event):
	if event is InputEventMouseButton:
		if event.is_pressed():
			rotating = true
		
		if event.is_released():
			rotating = false
	
	if event is InputEventMouseMotion and rotating:
		var delta = get_process_delta_time()
		var rel = event.relative
		
		$Character.rotate_y(rel.x * .5 * delta)

func _on_web_view_ipc_message(message):
	var data = JSON.parse_string(message)
	if data.type == "change_tab":
		var tween = get_tree().create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
		var position = Vector3(0, 0, 0)
		if data.tab == "hair" || data.tab == "eyes":
			position = Vector3(-0.23, 0.2, -1)
		tween.tween_property($Camera3D, "position", position, .8)
