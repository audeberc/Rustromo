extends Control

var player
var alien
var map
var label
var room_options_container
var selected_move_index = 0  
var possible_movements = []  
var map_overlay
var gameplay

func _ready():
	player = Player.new()
	player.initialize(3, 0, 4,100.0) 
	alien = Player.new()
	alien.initialize(100, 0, 4,100.0) 
	map = $GameMap  
	label = $VBoxContainer2/Label   
	room_options_container = $VBoxContainer2/VBoxContainer
	map_overlay = $MapBackground/MapOverlay
	gameplay = $Gameplay
	
	# Add rooms to the map
	var airlock = map.add_room("Airlock", "The airlock of the ship.",  0.495,0.9)
	var cockpit = map.add_room("Cockpit", "The control center of the ship.", 0.13, 0.3)
	var computer_room = map.add_room("Computer Room", "Where the ship's computer is located.",0.2952,0.06)
	var dinner_room = map.add_room("Dinner Room", "Where the crew eats their meals.",0.475,0.31)
	var hub1_top_floor = map.add_room("Hub 1 Top Floor", "The top floor of Hub 1.", 0.75,0.31)
	var medical = map.add_room("Medical", "The medical bay of the ship.", 0.75,0.615)
	var hub2_top_floor = map.add_room("Hub 2 Top Floor", "The top floor of Hub 2.", 0.486,0.615)
	var barracks = map.add_room("Barracks", "Where the crew sleeps.", 0.2,0.615)
	var hallway1 = map.add_room("Hallway (Cockpit to Computer Room)", "A connecting hallway from Cockpit to Computer Room.", 0.2952,0.21)
	var hallway2 = map.add_room("Hallway (Cockpit to Dinner Room)", "A connecting hallway from Cockpit to Dinner Room.", 0.3, 0.3)
	var hallway3 = map.add_room("Hallway (Dinner Room to Hub 1 Top Floor)", "A connecting hallway from Dinner Room to Hub 1 Top Floor.", 0.6,0.31)
	var hallway4 = map.add_room("Hallway (Hub 1 Top Floor to Medical)", "A connecting hallway from Hub 1 Top Floor to Medical.", 0.75, 0.45)
	var hallway5 = map.add_room("Hallway (Medical to Hub 2 Top Floor)", "A connecting hallway from Medical to Hub 2 Top Floor.", 0.67,0.615)
	var hallway6 = map.add_room("Hallway (Hub 2 Top Floor to Barracks)", "A connecting hallway from Hub 2 Top Floor to Barracks.", 0.34,0.615)
	var hallway7 = map.add_room("Hallway (Hub 2 Top Floor to Airlock)", "A connecting hallway from Hub 2 Top Floor to Airlock.", 0.485,0.78)

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
	# Move the player to room 1
	player.move_to_room(cockpit)
	alien.move_to_room(airlock)
	
	player.add_item('flamethrower', 5	)

	# Display the initial room info
	display_room_info()

	# Update the map overlay with the initial room coordinates
	var current_room_coordinates = map.get_room_coordinates(player.get_current_room_index())
	var alien_room_coordinates = map.get_room_coordinates(alien.get_current_room_index())
 
	map_overlay.update_current_room_coordinates(current_room_coordinates[0], current_room_coordinates[1])
	map_overlay.update_alien_coordinates(alien_room_coordinates[0], alien_room_coordinates[1])

func _process(delta):
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
	
	gameplay.handle_selected_item(map, player, alien, possible_movements[selected_move_index])
	selected_move_index = 0
	display_room_info()


# Utility function: Wraps the selection index around the list
func wrap(value, min_value, max_value):
	return (value - min_value) % (max_value - min_value) + min_value
