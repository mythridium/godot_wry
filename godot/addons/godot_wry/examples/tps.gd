extends Node3D

func _ready():
	#$WebView.set_process_input(false)
	#$WebView.set_process_unhandled_input(false)
	#$WebView.set_process_unhandled_key_input(false)
	
	await get_tree().create_timer(2).timeout
	$WebView.eval("ipc.postMessage(JSON.stringify({ type: 'eval', message: Math.PI }))")

func _on_web_view_ipc_message(message):
	var data = JSON.parse_string(message)
	print(data)
