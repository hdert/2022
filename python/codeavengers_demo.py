from tkinter import *

root = Tk()
root.title("Goal Tracker")

# Create and set the message text variable
message_text = StringVar()
message_text.set(
    "Welcome! You can deposit or withdraw money and see your progress towards your goals."
)

# Create the message label and add it to the window using pack()
message_label = Label(root, textvariable=message_text, wraplength=250)
message_label.pack()

#Create a PhotoImage()
# neutral_image = PhotoImage(file="/images/python/neutral.png")

#Create a new Label using the PhotoImage and pack it into the GUI
# image_label = Label(root, image=neutral_image)
# image_label.pack()

# Create and set the account details variable
account_details = StringVar()
account_details.set("Savings: $500 - 25% of $2000 goal \nTotal balance: $500")

# Create the details label and pack it into the GUI
details_label = Label(root, textvariable=account_details)
details_label.pack()

# Run the mainloop
root.mainloop()
