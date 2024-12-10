extends TextureRect
var png_files: Array = []
var change_interval: float = 1.0 # Change every second
var time_accumulated: float = 0.0
var current_index: int = 0

func _ready() -> void:
	var dir_path = "res://bg_frames/"

	var dir = DirAccess.open(dir_path)

	if dir:
		print("Directory opened successfully:", dir_path)
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
				if not dir.current_is_dir() and file_name.ends_with(".png"):
					png_files.append(dir_path + file_name)
				file_name = dir.get_next()
	
	print(png_files)
	if png_files.size() == 0:
		print("No PNG files found in the folder.")
	else:

		texture = load(png_files[current_index])

func _process(delta: float) -> void:

	time_accumulated += delta
	
	if time_accumulated >= change_interval:
		time_accumulated = 0.0 # 
		current_index = (current_index + 1) % png_files.size()
		texture = load(png_files[current_index])
