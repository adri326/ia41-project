import tkinter as tk

# Modified version of https://stackoverflow.com/a/22837522
class ResizingCanvas(tk.Canvas):
    def __init__(self, parent, **kwargs):
        tk.Canvas.__init__(self, parent, **kwargs)
        self.bind("<Configure>", self._on_resize)
        self.height = self.winfo_reqheight()
        self.width = self.winfo_reqwidth()
        self.on_resize = lambda x: x

    def _on_resize(self, event):
        self.width = event.width
        self.height = event.height
        # resize the canvas
        self.config(width=self.width, height=self.height)

        self.on_resize(self)
