import tkinter as tk

# Modified version of https://stackoverflow.com/a/22837522
class ResizingCanvas(tk.Canvas):
    def __init__(self, parent, **kwargs):
        tk.Canvas.__init__(self, parent, **kwargs)
        self.bind("<Configure>", self._on_resize)
        self.width = self.winfo_reqwidth()
        self.height = self.winfo_reqheight()
        self.on_resize = lambda x: x

    def _on_resize(self, event):
        self.create_rectangle(0, 0, event.width, event.height, fill="#ffffff")
        self.width = event.width - 2
        self.height = event.height - 2
        # resize the canvas
        self.config(width=self.width, height=self.height)

        self.on_resize(self)
