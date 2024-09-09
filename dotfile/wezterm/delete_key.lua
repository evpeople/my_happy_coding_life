local wezterm = require("wezterm")
local act = wezterm.action
local module = {}

function module.apply_to_config(config)
	local keys = {
		{ key = "Enter", mods = "ALT", action = wezterm.action.DisableDefaultAssignment },
	}

	if not config.keys then
		config.keys = {}
	end
	for _, k in ipairs(keys) do
		table.insert(config.keys, k)
	end
end

return module
