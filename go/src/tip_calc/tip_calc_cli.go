package main

import (
	"flag"
	"fmt"
	"log"
	"math"
	"strconv"

	"github.com/gotk3/gotk3/glib"
	"github.com/gotk3/gotk3/gtk"
)

const UIMain = "tip_calc_window.glade"

func graphicalUI() {
	// Create Gtk Application, change appID to your application domain name reversed.
	const appID = "fr.abrivard.tip_calc"
	application, err := gtk.ApplicationNew(appID, glib.APPLICATION_FLAGS_NONE)
	// Check to make sure no errors when creating Gtk Application
	if err != nil {
		log.Fatal("Could not create application.", err)
	}

	application.Connect("activate", func() {
		builder, err := gtk.BuilderNew()
		if err != nil {
			log.Fatalln("Couldn't make builder:", err)
		}

		err = builder.AddFromFile(UIMain)
		if err != nil {
			log.Fatalln("Couldn't add UI XML to builder:", err)
		}

		var obj glib.IObject

		obj, err = builder.GetObject("TipScale")
		scale := obj.(*gtk.Scale)
		scaleAdj := scale.GetAdjustment()

		obj, err = builder.GetObject("TipEntry")
		tipEntry := obj.(*gtk.Entry)

		_ = glib.BindProperty(scaleAdj.Object,"value",tipEntry.Object,"text",glib.BINDING_DEFAULT|glib.BINDING_BIDIRECTIONAL|glib.BINDING_SYNC_CREATE)

		obj, err = builder.GetObject("BillEntry")
		billEntry := obj.(*gtk.Entry)

		obj, err = builder.GetObject("TipAmountLbl")
		tipAmount := obj.(*gtk.Label)

		obj, err = builder.GetObject("TotalBillLbl")
		totalBill := obj.(*gtk.Label)

		obj, err = builder.GetObject("ComputeCmd")
		button := obj.(*gtk.Button)

		_, err = button.Connect("clicked", func() {
			billText, _ := billEntry.GetText()
			tipText, _ := tipEntry.GetText()

			if bill, err := strconv.ParseFloat(billText, 64); err == nil && bill >= 0 {
				if tipPercent, err := strconv.ParseFloat(tipText, 64); err == nil && tipPercent >= 0 {
					tip := math.Round(tipPercent*bill) / 100
					tipAmount.SetLabel(fmt.Sprintf("The tip is %.2f", tip))
					totalBill.SetLabel(fmt.Sprintf("The total is %.2f", bill+tip))
					return
				}
			}

			tipAmount.SetLabel("Please enter numbers >= 0")
			totalBill.SetLabel("")
		})

		obj, err = builder.GetObject("MainWindow")
		wnd := obj.(*gtk.ApplicationWindow)
		wnd.ShowAll()
		application.AddWindow(wnd)
	})
	// Run Gtk application
	application.Run([]string{})
}

func readPositiveFloat() float64 {
	for {
		var line string
		_, err := fmt.Scanln(&line)
		if err != nil {
			panic(err)
		}

		if value, err := strconv.ParseFloat(line, 64); err == nil && value >= 0 {
			return value
		}

		fmt.Print("Please enter a value >= 0 : ")
	}
}

func consoleUI() {
	fmt.Print("What is the bill? ")
	bill := readPositiveFloat()

	fmt.Print("What is the tip percentage? ")
	tipPercent := readPositiveFloat()

	tip := math.Round(tipPercent*bill) / 100

	fmt.Printf("The tip is %.2f\nThe total is %.2f\n", tip, bill+tip)
}

func main() {
	uiPtr := flag.String("ui","c","c for console, g for GUI")
	flag.Parse()

	if *uiPtr=="c" {
		consoleUI()
	} else {
		graphicalUI()
	}
}
