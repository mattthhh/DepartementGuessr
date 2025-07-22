import csv
import random
import tkinter as tk
from tkinter import messagebox


class DepartmentGame:
    def __init__(self, master, data):
        self.master = master
        self.data = data
        self.filtered = data
        self.current = None

        self.start_var = tk.StringVar()
        self.end_var = tk.StringVar()

        tk.Label(master, text="Start code:").grid(row=0, column=0)
        tk.Entry(master, textvariable=self.start_var).grid(row=0, column=1)
        tk.Label(master, text="End code:").grid(row=1, column=0)
        tk.Entry(master, textvariable=self.end_var).grid(row=1, column=1)
        tk.Button(master, text="Set Range", command=self.set_range).grid(row=2, column=0, columnspan=2)

        self.code_label = tk.Label(master, font=("Arial", 24))
        self.code_label.grid(row=3, column=0, columnspan=2, pady=10)

        tk.Label(master, text="Department name:").grid(row=4, column=0)
        self.answer_entry = tk.Entry(master)
        self.answer_entry.grid(row=4, column=1)

        tk.Button(master, text="Submit", command=self.check_answer).grid(row=5, column=0, columnspan=2, pady=5)
        self.message_label = tk.Label(master, text="")
        self.message_label.grid(row=6, column=0, columnspan=2)

        self.next_question()

    def set_range(self):
        start = self.start_var.get().strip()
        end = self.end_var.get().strip()
        codes = [d['code'] for d in self.data]
        if start not in codes or end not in codes:
            messagebox.showerror("Error", "Invalid codes")
            return
        start_index = codes.index(start)
        end_index = codes.index(end)
        if start_index > end_index:
            start_index, end_index = end_index, start_index
        self.filtered = self.data[start_index:end_index + 1]
        self.next_question()

    def next_question(self):
        self.current = random.choice(self.filtered)
        self.code_label.config(text=self.current['code'])
        self.answer_entry.delete(0, tk.END)
        self.message_label.config(text="")

    def check_answer(self):
        answer = self.answer_entry.get().strip().lower()
        if answer == self.current['name'].lower():
            self.message_label.config(text="Correct!")
        else:
            self.message_label.config(
                text=f"Incorrect. {self.current['code']} = {self.current['name']}")
        self.master.after(1500, self.next_question)


def load_data(path):
    data = []
    with open(path, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            data.append({'code': row['code_departement'],
                         'name': row['nom_departement']})
    return data


if __name__ == '__main__':
    data = load_data('departements.csv')
    root = tk.Tk()
    root.title("Departement Guessr")
    DepartmentGame(root, data)
    root.mainloop()
