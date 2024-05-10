// ==UserScript==
// @id             iitc-plugin-side-tool-selector@eccenux
// @name           IITC plugin: Side tools selector.
// @category       Info
// @version        0.0.1
// @description    [0.0.1] Allows to choose which side tools should be temporarly hidden.
// @namespace      https://github.com/jonatkins/ingress-intel-total-conversion
// @include        https://*.ingress.com/intel*
// @include        http://*.ingress.com/intel*
// @match          https://*.ingress.com/intel*
// @match          http://*.ingress.com/intel*
// @include        https://*.ingress.com/mission/*
// @include        http://*.ingress.com/mission/*
// @match          https://*.ingress.com/mission/*
// @match          http://*.ingress.com/mission/*
// @grant          none
// @updateURL      https://github.com/Eccenux/iitc-plugin-side-tool-selector/raw/master/side-tool-selector.meta.js
// @downloadURL    https://github.com/Eccenux/iitc-plugin-side-tool-selector/raw/master/side-tool-selector.user.js
// ==/UserScript==

// ensure plugin framework is there, even if iitc is not yet loaded
if(typeof window.plugin !== 'function') window.plugin = function() {};

(function () {

	// use own namespace for plugin
	window.plugin.sideToolSelector = function() {};

	/**
	 * Setup GUI.
	 */
	window.plugin.sideToolSelector.setup  = function() {

		// add menu/options button
		$('<a>Tool selector</a>').appendTo('#toolbox').click(()=>{
			window.plugin.sideToolSelector.openOptions();
		});
	};

	/**
	 * Sidebar tool (helper class).
	 */
	class Tool {
		constructor(toolKey, toolNode) {
			this.key = toolKey;
			this.node = toolNode;
			this.hidden = toolNode.style.display === 'none';
		}
		toggle(hide) {
			if (typeof hide == 'undefined') {
				hide = !this.hidden;
			}

			if (hide) {
				this.node.style.display = 'none';
				this.hidden = true;
			} else {
				this.node.style.display = 'block';
				this.hidden = false;
			}
		}
	}
	window.plugin.sideToolSelector.Tool = Tool;

	/**
	 * Get current tools.
	 */
	window.plugin.sideToolSelector.getTools = function() {
		let toolList = document.querySelectorAll('.leaflet-top.leaflet-left > .leaflet-control');
		let tools = {};
		for (const toolNode of toolList) {
			let added = false;
			toolNode.className.replace(/leaflet-control-(\S+)/, (a, toolKey) => {
				tools[toolKey] = new window.plugin.sideToolSelector.Tool(toolKey, toolNode);
				added = true;
			});
			// try again (e.g. for draw tools)
			if (!added) {
				let toolKey = toolNode.className.replace(/leaflet-(control|bar)-?/g, '').trim();
				tools[toolKey] = new window.plugin.sideToolSelector.Tool(toolKey, toolNode);
			}
		}
		console.log('tools: ', tools);
		return tools;
	}

	/**
	 * Show main dialog.
	 */
	window.plugin.sideToolSelector.openOptions = function () {
		let tools = window.plugin.sideToolSelector.getTools();
		
		// body
		let tbody = '';
		for (const key in tools) {
			const tool = tools[key];
			tbody += `<tr>
				<td><label><input type="checkbox" value="1" data-key="${tool.key}" class="now" ` + (tool.hidden ? '' : 'checked') + `> ${tool.key}</label></td>
				<td>TODO</td>
			</tr>`;
		}

		// structure
		let html = `Tools
			<table>
				<thead>
					<tr><th>Now (temporary)</th><th>Auto-hide</th></tr>
				</thead>
				<tbody>${tbody}</tbody>
			</table>
		`;

		// show
		dialog({
			html: html,
			dialogClass: 'ui-dialog-sideToolSelector',
			title: 'Show/hide tools'
		});

		// actions
		document.querySelectorAll('.ui-dialog-sideToolSelector tbody .now').forEach((input) => {
			input.addEventListener('click', (event) => {
				let toolKey = input.getAttribute('data-key');
				if (toolKey in tools) {
					let tool = tools[toolKey];
					tool.toggle(!input.checked);
				} else {
					console.warn('tool not found: ', toolKey);
				}
			});
		});
	};

})();
// PLUGIN END //////////////////////////////////////////////////////////
function wrapper(plugin_info) {
	var setup =  window.plugin.sideToolSelector.setup;	

	setup.info = plugin_info; //add the script info data to the function as a property
	if(!window.bootPlugins) window.bootPlugins = [];
	window.bootPlugins.push(setup);
	// if IITC has already booted, immediately run the 'setup' function
	if(window.iitcLoaded && typeof setup === 'function') setup();
} // wrapper end

// inject code into site context
var script = document.createElement('script');
var info = {};
if (typeof GM_info !== 'undefined' && GM_info && GM_info.script) info.script = { version: GM_info.script.version, name: GM_info.script.name, description: GM_info.script.description };
script.appendChild(document.createTextNode('('+ wrapper +')('+JSON.stringify(info)+');'));
(document.body || document.head || document.documentElement).appendChild(script);
