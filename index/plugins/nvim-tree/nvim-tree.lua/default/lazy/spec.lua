return {
  "nvim-tree/nvim-tree.lua",
  commit = "%commit",
  lazy = false,
  dependencies = {
    "nvim-tree/nvim-web-devicons",
  },
  config = function()
    require("nvim-tree").setup {}
  end,
}
