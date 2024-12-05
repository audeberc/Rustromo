extends Control

var player
var alien
var map
var label
var player_label
var room_options_container
var selected_move_index = 0  
var possible_movements = []  
var map_overlay
var gameplay
var cockpit
var lair
var game_over_label
var replay_button
var game_over = false
var map_background 

func _ready():
	player = Player.new()
	player.initialize(3, 0, 4, 100.0) 
	alien = Player.new()
	alien.initialize(100, 0, 4, 100.0) 
	map = $GameMap  
	label = $VBoxContainer2/HBox_room_info/room_info_label   
	player_label = $VBoxContainer2/HBox_player_info2/player_info_label   
	room_options_container = $VBoxContainer2/VBoxContainer
	map_overlay = $MapBackground/MapOverlay
	map_background = $MapBackground

	gameplay = $Gameplay
	game_over_label = $GameOverLabel
	replay_button = $ReplayButton
	
	# Add rooms to the map - top floor- 
	lair = map.add_room("Lair", "The monster is nesting here.",0.738, 0.875)
	
	var airlock = map.add_room("Airlock", "The airlock of the ship.", 0.448,0.544)
	cockpit = map.add_room("Cockpit", "The control center of the ship.",0.118,0.190)
	var computer_room = map.add_room("Computer Room", "Where the ship's computer is located.",0.290, 0.049)
	var dinner_room = map.add_room("Dinner Room", "Where the crew eats their meals.",0.453, 0.196)
	var hub1_top_floor = map.add_room("Hub 1 Top Floor", "The top floor of Hub 1.", 0.738, 0.191)
	var medical = map.add_room("Medical", "The medical bay of the ship.", 0.741,0.371)
	var hub2_top_floor = map.add_room("Hub 2 Top Floor", "The top floor of Hub 2.",0.453, 0.371)
	var barracks = map.add_room("Barracks", "Where the crew sleeps.", 0.213, 0.372)
	var hallway1 = map.add_room("Hallway Cockpit <-> Computer Room", "", 0.290, 0.134)
	var hallway2 = map.add_room("Hallway Cockpit <-> Dinner Room", "", 0.290, 0.190)
	var hallway3 = map.add_room("Hallway Dinner Room <-> Hub1 Top", "", 0.594, 0.191)
	var hallway4 = map.add_room("Hallway Hub1 Top <-> Medical", "", 0.738, 0.280)
	var hallway5 = map.add_room("Hallway Medical <-> Hub2 Top", "", 0.592, 0.372)
	var hallway6 = map.add_room("Hallway Hub2 Top <-> Barracks", "", 0.332, 0.372)
	var hallway7 = map.add_room("Hallway Hub2 Top <-> Airlock", "", 0.450, 0.463)
	# Add rooms to the map - bottom floor- 
	var hub1_bottom_floor = map.add_room("Hub 1 Bottom Floor", "The bottom floor of Hub 1.",0.736, 0.692)
	var engines = map.add_room("Engines room", "The engines control room.",0.918, 0.695)
	var hub2_bottom_floor = map.add_room("Hub 2 Bottom Floor", "The bottom floor of Hub 2.",0.453, 0.875)
	var storage = map.add_room("Storage", "Storage room of the ship.",0.215, 0.874 )
	var ladder_1 = map.add_room("Ladder 1", "Ladder of Hub 1.", 0.908, 0.441)
	var ladder_2 = map.add_room("Ladder 2", "Ladder of Hub 2.", 0.557, 0.627)
	var hallway8 = map.add_room("Hallway Hub1 Bottom <-> Engines", "", 0.839, 0.691)
	var hallway9 = map.add_room("Hallway Hub1 Bottom <-> Lair", "", 0.735, 0.776)
	var hallway10 = map.add_room("Hallway Lair <-> Hub2 Bottom", "", 0.581, 0.872)
	var hallway11 = map.add_room("Hallway Hub2 Bottom <-> Storage", "", 0.340, 0.874)
	# Connect rooms through hallways
	map.connect_rooms(cockpit, hallway1)
	map.connect_rooms(hallway1, computer_room)
	map.connect_rooms(cockpit, hallway2)
	map.connect_rooms(hallway2, dinner_room)
	map.connect_rooms(dinner_room, hallway3)
	map.connect_rooms(hallway3, hub1_top_floor)
	map.connect_rooms(hub1_top_floor, hallway4)
	map.connect_rooms(hallway4, medical)
	map.connect_rooms(medical, hallway5)
	map.connect_rooms(hallway5, hub2_top_floor)
	map.connect_rooms(hub2_top_floor, hallway6)
	map.connect_rooms(hallway6, barracks)
	map.connect_rooms(hub2_top_floor, hallway7)
	map.connect_rooms(hallway7, airlock)
	map.connect_rooms(hub1_bottom_floor, hallway8)
	map.connect_rooms(hallway8, engines)
	map.connect_rooms(hub1_bottom_floor, hallway9)
	map.connect_rooms(hallway9, lair)
	map.connect_rooms(lair, hallway10)
	map.connect_rooms(hallway10, hub2_bottom_floor)
	map.connect_rooms(hub2_bottom_floor, hallway11)
	map.connect_rooms(hallway11, storage)
	map.connect_rooms(hub1_top_floor, ladder_1)
	map.connect_rooms(hub1_bottom_floor, ladder_1)
	map.connect_rooms(hub2_top_floor, ladder_2)
	map.connect_rooms(hub2_bottom_floor, ladder_2)
	# Move the player to room 1
	player.move_to_room(cockpit)
	alien.move_to_room(lair)
	
	player.add_item('flamethrower', 1	)
		
	player.add_item('flare', 2	)
	# Display the initial room info
	display_room_info()

	# Update the map overlay with the initial room coordinates
	var current_room_coordinates = map.get_room_coordinates(player.get_current_room_index())
	var alien_room_coordinates = map.get_room_coordinates(alien.get_current_room_index())
 
	map_overlay.update_current_room_coordinates(current_room_coordinates[0], current_room_coordinates[1])
	map_overlay.update_alien_coordinates(alien_room_coordinates[0], alien_room_coordinates[1])

	# Hide game over elements initially
	game_over_label.visible = false
	replay_button.visible = false
	replay_button.connect("pressed", Callable(self, "_on_replay_button_pressed"))

func _process(delta):
	if game_over:
		if Input.is_action_just_pressed("ui_accept"):
			_on_replay_button_pressed()
	else:
		# Handle input for selecting rooms
		if Input.is_action_just_pressed("ui_down"):
			change_selection(1)
		elif Input.is_action_just_pressed("ui_up"):
			change_selection(-1)
		elif Input.is_action_just_pressed("ui_accept"):
			move_to_selected_room()

# Update the displayed room info and options
func display_room_info():
	# Clear previous room options
	for child in room_options_container.get_children():
		room_options_container.remove_child(child)
		child.queue_free()

	# Update the map overlay with the current room coordinates
	var current_room_coordinates = map.get_room_coordinates(player.get_current_room_index())
	map_overlay.update_current_room_coordinates(current_room_coordinates[0], current_room_coordinates[1])
	var alien_room_coordinates = map.get_room_coordinates(alien.get_current_room_index())

	map_overlay.update_alien_coordinates(alien_room_coordinates[0], alien_room_coordinates[1])
	# Display current room info
	label.text = gameplay.create_room_info_text(map, player)
	player_label.text = gameplay.create_player_info_text(player)
	# Get connected rooms and display as selectable options
	possible_movements = gameplay.get_possible_movements(map, player, alien)


	for i in range(possible_movements.size()):
		var room_label = Label.new()
		room_label.text = gameplay.parse_instruction(map, possible_movements[i])
		if i == selected_move_index:
			room_label.add_theme_color_override("font_color", Color(1, 1, 0))  # Highlight selected option
		else:
			room_label.add_theme_color_override("font_color", Color(0.177, 0.7, 0.168))
		room_options_container.add_child(room_label)

# Change the currently selected room
func change_selection(direction):
	selected_move_index = wrap(selected_move_index + direction, 0, possible_movements.size())
	display_room_info()  # Refresh display

# Move the player to the selected room
func move_to_selected_room():
	var result = gameplay.handle_selected_item(map, player, alien, possible_movements[selected_move_index])
	if result == "game_over":
		_on_game_over()
	selected_move_index = 0
	display_room_info()

# Utility function: Wraps the selection index around the list
func wrap(value, min_value, max_value):
	return (value - min_value) % (max_value - min_value) + min_value

func _on_game_over():
	game_over_label.visible = true
	replay_button.visible = true
	label.visible = false
	room_options_container.visible = false
	map_overlay.visible = false
	map_background.visible = false

	game_over = true

func _on_replay_button_pressed():
	game_over_label.visible = false
	replay_button.visible = false
	game_over = false
	label.visible = true
	room_options_container.visible = true
	map_overlay.visible = true
	map_background.visible = true
	reset_game()

func reset_game():
	player.initialize(3, 0, 4, 100.0)
	alien.initialize(100, 0, 4, 100.0)
	player.move_to_room(cockpit)
	alien.move_to_room(lair)
	player.add_item('flamethrower', 5)
	display_room_info()
	selected_move_index = 0
	var current_room_coordinates = map.get_room_coordinates(player.get_current_room_index())
	var alien_room_coordinates = map.get_room_coordinates(alien.get_current_room_index())
	map_overlay.update_current_room_coordinates(current_room_coordinates[0], current_room_coordinates[1])
	map_overlay.update_alien_coordinates(alien_room_coordinates[0], alien_room_coordinates[1])
